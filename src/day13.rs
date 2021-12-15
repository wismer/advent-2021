use std::fs::read_to_string;
use std::fmt;
pub fn solve() {
    let input = read_to_string("./day13.txt")
        .unwrap();

    let mut coordinates: Vec<&str> = vec![];
    let mut instructions: Vec<&str> = vec![];

    for line in input.lines() {
        if line.len() == 0 {
            continue;
        } else if line.contains(",") {
            coordinates.push(line);
        } else {
            instructions.push(line);
        }
    }

    let mut graph = Graph::new(parse_coordinates(&coordinates));
    for instruction in parse_instructions(&instructions) {
        graph.fold(&instruction);
        graph.print_map((-1, -1));
    }
}

fn parse_coordinates(input: &Vec<&str>) -> Vec<Point> {
    input.iter().map(|l| {
        let xl: Vec<usize> = l.split(",").map(|n| n.parse().unwrap()).collect();
        Point(xl[0], xl[1])
    }).collect()
}

fn parse_instructions(input: &Vec<&str>) -> Vec<Instruction> {
    input.iter().map(|l| {
        let xl: Vec<&str> = l.split("=").collect();
        match xl[0] {
            "fold along x" => Instruction(xl[1].parse().unwrap(), "x"),
            "fold along y" => Instruction(xl[1].parse().unwrap(), "y"),
            _ => panic!("unparseable! : {:?}", xl)
        }
    }).collect()
}

#[derive(Clone, Copy, Debug)]
struct Instruction(usize, &'static str);
#[derive(Clone, Copy, Debug)]
struct Point(usize, usize);

struct Graph {
    pub graph: Vec<Vec<Option<Point>>>,
    size_x: usize,
    size_y: usize
}

impl Graph {
    pub fn new(coordinates: Vec<Point>) -> Self {
        let size_x = coordinates.iter().map(|pt| pt.0).max().unwrap();
        let size_y = coordinates.iter().map(|pt| pt.1).max().unwrap();
        let mut graph: Vec<Vec<Option<Point>>> = vec![];
        let mut rows: Vec<Option<Point>> = vec![];
        // resize rows to have x number of columns
        rows.resize(size_x + 1, None);
        // resize graph to have y number of rows
        graph.resize(size_y + 1, rows);
        
        for coordinate in coordinates {
            let row = graph.get_mut(coordinate.1).unwrap();
            row[coordinate.0] = Some(coordinate);
        }

        Graph {
            size_x: size_x + 1,
            size_y: size_y + 1,
            graph: graph
        }
    }

    pub fn fold(&mut self, instruction: &Instruction) {
        match instruction.1 {
            "x" => self.fold_left(instruction.0),
            "y" => self.fold_up(instruction.0),
            _ => panic!("should not happen")
        }
    }

    pub fn print_map(&self, fold_pt: (isize, isize)) {
        let mut buffer = String::new();

        for y in 0..self.size_y {
            if fold_pt.1 >= 0 && fold_pt.1 as usize == y {
                buffer.push_str("->");
            } else {
                buffer.push_str("  ");
            }
            for x in 0..self.size_x {
                if self.graph[y][x].is_some() {
                    buffer.push_str("*");
                } else {
                    buffer.push_str(".");
                }
            }
            buffer.push_str("\n");
        }
        for line in buffer.lines() {
            println!("{}", line);
        }
        println!("\n");
    }

    fn fold_left(&mut self, column: usize) {
        for row in self.graph.iter_mut() {
            // how many columns are being folded in
            let col_num = row.len();
            // println!("to_move: {}, size: {}, column: {}", columns_to_move, row.len(), column);
            let (mut col_start, mut col_end) = if column >= self.size_x / 2 {
                (0, self.size_x - 1)
            } else {
                (0, self.size_x - 1)
            };
            while col_start != column && col_end != column {
                if row[col_end].is_some() {
                    row[col_start] = row[col_end];
                }
                col_start += 1;
                col_end -= 1;
            }

            row.resize(column, None);
        }
        self.size_x = self.graph[0].len();
        // self.print_map((-1, -1));


    }

    fn fold_up(&mut self, nth_row: usize) {
        let diff = self.size_y - nth_row;
        let (mut row_start, mut row_end) = if nth_row >= self.size_y / 2 {
            (0, self.size_y - 1)
        } else {
            (0, self.size_y - 1)
        };
        while row_start != nth_row && row_end != nth_row {
            // borrow the bottom row, but mutably borrow top row?
            let mut top_row = self.graph[row_start].clone();
            let bottom_row = &self.graph.get(row_end).unwrap();
            for (x, v) in bottom_row.iter().enumerate() {
                if v.is_some() {
                    top_row[x] = *v;
                }
            }

            self.graph[row_start] = top_row;
            row_start += 1;
            row_end -= 1;
        }
        self.graph.resize(nth_row - 1, vec![]);
        self.size_y = self.graph.len();
    }

    pub fn count_coordinates(&self) -> usize {
        let mut coords = 0;
        for row in &self.graph {
            for col in row {
                match col {
                    Some(_) => coords += 1,
                    None => {}
                }
            }
        }

        coords
    }
}


