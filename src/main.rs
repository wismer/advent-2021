use std::fmt;
use std::fs::{read_to_string, read};
use std::collections::{HashMap, HashSet};
mod day13;
mod day14;

fn main() {
    day13::solve();
    day14::solve();
    // day_one_part_one();
    // day_one_part_two();

    // day_two_part_one();
    // day_two_part_two();

    // day_three_part_one();
    // day_three_part_two();

    // // day_four_part_one();

    // day_five_part_one();

    // day_six();
    // day_seven();
    // day_eight();
    // day_nine();
    // day_ten();
    
}

fn day_two_part_one() {
    let data = read_to_string("./day2.txt").unwrap();
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for line in data.lines() {
        let instruction: Vec<&str> = line.split(" ").collect();
        let direction = instruction[0];
        let value: usize = instruction[1].parse().unwrap();

        match direction {
            "forward" => horizontal += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => unimplemented!()
        }
    }

    println!("depth: {}, horizontal: {}, aim: {}, result: {}", depth, horizontal, aim, horizontal * depth);
}

fn day_two_part_two() {
    let data = read_to_string("./day2.txt").unwrap();
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for line in data.lines() {
        let instruction: Vec<&str> = line.split(" ").collect();
        let direction = instruction[0];
        let value: usize = instruction[1].parse().unwrap();

        match direction {
            "forward" => {
                horizontal += value;
                depth += aim * value;
            },
            "down" => aim += value,
            "up" => aim -= value,
            _ => unimplemented!()
        }
    }
    println!("depth: {}, horizontal: {}, aim: {}, result: {}", depth, horizontal, aim, horizontal * depth);

}


fn day_one_part_one() {
    let data = read_to_string("./day1.txt").unwrap();
    let mut tally = 0;
    let mut previous: Option<usize> = None;
    for line in data.lines() {
        let num: usize = line.parse().unwrap();
        match previous {
            Some(n) => {
                if n < num {
                    tally += 1;
                }
            },
            None => {}
        }
        previous = Some(num)
    }

    println!("Day One Part One: {}", tally);

}

fn day_one_part_two() {
    let data: Vec<usize> = read_to_string("./day1.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    let mut tally = 0;
    let mut previous_sum: Option<usize> = None;
    let mut chunk_iter = data.windows(3);

    loop {
        // grab the next chunk
        let next_chunk: usize = match chunk_iter.next() {
            Some(chunk) => {
                if chunk.len() != 3 {
                    break;
                }
                chunk.iter().sum()
            },
            None => break
        };

        match previous_sum {
            Some(n) => {
                if n < next_chunk {
                    tally += 1;
                }
            },
            None => {}
        }
        previous_sum = Some(next_chunk);
    }

    println!("Total: {}", tally);
}

fn day_three_part_one() {
    let mut gamma = 0;
    let data: Vec<usize> = read_to_string("./day3.txt")
        .unwrap()
        .lines()
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .collect();

    let flags: [usize; 12] = [
        1 << 12,
        1 << 11,
        1 << 10,
        1 << 9,
        1 << 8,
        1 << 7,
        1 << 6,
        1 << 5,
        1 << 4,
        1 << 3,
        1 << 2,
        1
    ];
    let line_count = data.len() / 2;
    for flag in flags {
        let mut count = 0;
        for set in &data {
            if set & flag != 0 {
                count += 1;
            }
        }

        if count >= line_count {
            println!("{:#07b}\n{:#07b}\n-----\n{:#07b}\n", gamma, flag, gamma ^ flag);
            gamma = gamma ^ flag;
        }
    }
    let epsilon = !gamma & 0b111111111111;
    println!("Gamma: {}, Epsilon: {}, power consumption: {}", gamma, epsilon, gamma * epsilon);

}

fn day_three_part_two() {
    let data: Vec<usize> = read_to_string("./day3.txt")
        .unwrap()
        .lines()
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .collect();

    let o2 = resolve_oxygen_rating(&data, 11, true);
    let co2 = resolve_oxygen_rating(&data, 11, false);
    println!("oxygen: {} co2: {}, result: {}", o2, co2, o2 * co2);
}

fn resolve_oxygen_rating(values: &Vec<usize>, bit_position: usize, find_majority: bool) -> usize {
    let flag = 1 << bit_position;
    let mut leading_bits: Vec<usize> = vec![];
    let mut nonleading_bits: Vec<usize> = vec![];
    for set in values {
        if flag & set != 0 {
            // yes for O2, no for CO2
            leading_bits.push(*set)
        } else {
            nonleading_bits.push(*set);
        }
    }

    let leading_bits_size = leading_bits.len();
    let nonleading_bits_size = nonleading_bits.len();
    let data_to_process: Vec<usize>;
    if find_majority && leading_bits_size >= nonleading_bits_size {
        data_to_process = leading_bits;
    } else if find_majority {
        data_to_process = nonleading_bits;
    } else if leading_bits_size >= nonleading_bits_size {
        data_to_process = nonleading_bits
    } else {
        data_to_process = leading_bits;
    }
    if data_to_process.len() >= 2 {
        resolve_oxygen_rating(&data_to_process, bit_position - 1, find_majority)
    } else {
        return data_to_process[0]
    }
}

#[derive(Debug)]
struct Line {
    start: (usize, usize),
    end: (usize, usize)
}

impl Line {
    pub fn parse(line: &str) -> (usize, usize) {
        let parsed: Vec<usize> = line.split(",").map(|l| l.parse().unwrap()).collect();
        (parsed[0], parsed[1])
    }
}

fn day_five_part_one() {
    let data = read_to_string("./day5.txt").unwrap();
    let mut lines: Vec<Line> = vec![];
    let mut max_y = 0;
    let mut max_x = 0;
    for line in data.lines() {
        let parsed_line: Vec<(usize, usize)> = line.split(" -> ").map(|line| Line::parse(line)).collect();
        let line = Line { start: parsed_line[0], end: parsed_line[1] };
        if line.end.0 >= max_x {
            max_x = line.end.0;
        }
        if line.end.1 >= max_y {
            max_y = line.end.1;
        }
        lines.push(line);
    }

    let line_count = lines.len();
    let mut all_coords: Vec<usize> = vec![];

    for line in lines {
        let (line_start_x, line_start_y) = line.start;
        let (line_end_x, line_end_y) = line.end;
        if line_start_x == line_end_x {
            for y in line_start_y..line_end_y {
                match all_coords.get_mut(line_end_x + y) {
                    Some(coord) => *coord += 1,
                    None => {}
                }
            }
        } else {
            println!("sad x: {}", line_start_x * max_x);

            let y = line_start_y * max_y;
            for x in line_start_x..line_end_x {
                match all_coords.get_mut(x + y) {
                    Some(coord) => *coord += 1,
                    None => {}
                }
            }
        }
    }
    println!("{:?}", all_coords);

}

fn day_six() {
    let mut inputs: Vec<usize> = read_to_string("./day6.txt")
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut values: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
    for v in inputs {
        values[v as usize] += 1;
    }

    for end in [80, 256] {
        let mut current = values.clone();
        println!("before: {:?}", current);
        for _ in 0..end {
            let mut new_state: Vec<usize> = current.clone();
            let zeroes = new_state[0];
            new_state.rotate_left(1);
            new_state[6] += zeroes;
            // println!("before: {:?}", current);
            // println!("after : {:?}\n", new_state);
            current = new_state;
        }
        let sum: usize = current.iter().sum();
        println!("end: {}, sum: {}", end, sum);
    }
}

fn day_seven() {
    let inputs: Vec<usize> = read_to_string("./day7.txt")
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let p1 = {
        let max = *inputs.iter().max().unwrap();
        let min = *inputs.iter().min().unwrap();
        let mut movements: Vec<usize> = vec![];

        for i in min..max {
            let mut subtotal = 0;
            for crab in &inputs {
                let diff = if i > *crab {
                    i - crab
                } else {
                    crab - i
                };
                subtotal += diff;
            }
            movements.push(subtotal);
        }

        *movements.iter().min().unwrap()
    };

    let p2 = {
        let max = *inputs.iter().max().unwrap();
        let min = *inputs.iter().min().unwrap();
        let mut movements: Vec<usize> = vec![];

        for i in min..max {
            let mut subtotal = 0;
            for crab in &inputs {
                let diff = if i > *crab {
                    i - crab
                } else {
                    crab - i
                };
                subtotal += calculate_cost(diff);
            }
            movements.push(subtotal);
        }

        *movements.iter().min().unwrap()

    };

    println!("p1: {}, p2: {}", p1, p2);
}

fn calculate_cost(mut distance: usize) -> usize {
    let mut cost = 0;
    while distance != 0 {
        cost += distance;
        distance -= 1;
    }

    cost
}

// enum Segment {
//     A,
//     B,
//     C,
//     D,
//     E,
//     F,
//     G
// }
#[derive(Debug, Clone)]
struct SegmentPiece {
    pub raw_segment: String,
    pub number: Option<usize>
}

fn day_eight() {
    // entries consist of:
    //   10 unique signal patterns
    //   delimited by |
    //   final output is a 4 digit value
    // ex 
    // 7 uniquely uses 3 segments
    // 4 uniquely uses 4 segments
    //   for 4, it's like if "eafb" is the signal, then "bcdf" are the segments
    //   deduction should be used?
    let input = read_to_string("./day8.txt")
        .unwrap();

    let pt1 = {

        let mut total = 0;
        for line in input.lines() {
            let parsed: Vec<Vec<String>> = line
                .split(" | ")
                .map(|x| {
                    let p: Vec<String> = x.split(" ")
                        .map(|x| x.to_string())
                        .collect();
                    p
                })
                .collect();
            total += count_unique_digits(&parsed[1]);
            
        }
        total
    };

    let pt2 = {
        // diff("cfgedb".to_string(), "dgc".to_string())
        let mut total: usize = 0;
        for line in input.lines() {
            let mut parsed: Vec<Vec<String>> = line
                .split(" | ")
                .map(|x| {
                    let p: Vec<String> = x.split(" ")
                        .map(|x| x.to_string())
                        .collect();
                    p
                })
                .collect();

            {
                let mut seg_hash: HashMap<usize, HashSet<_>> = HashMap::new();
                for segment in &parsed[0] {
                    let len = segment.len();
                    match len {
                        2 => seg_hash.insert(1, segment.chars().collect()),
                        3 => seg_hash.insert(7, segment.chars().collect()),
                        4 => seg_hash.insert(4, segment.chars().collect()),
                        8 => seg_hash.insert(8, segment.chars().collect()),
                        _ => None
                    };
                }
                let mut output = String::new();
                for segment in &parsed[1] {
                    let decoded: String = decode_segment(segment, &mut seg_hash).to_string();
                    output.push_str(&decoded);
                }
                total += output.parse::<usize>().unwrap();
                println!("{:?}", output);
            }
        }
        println!("total: {}", total);
    };

    println!("part 1: {}\npart 2: {}", pt1, 1);
}

fn decode_segment(segment: &str, seg_hash: &mut HashMap<usize, HashSet<char>>) -> usize {
    let mut set: HashSet<_> = segment.chars().collect();
    let one = seg_hash.get(&1).unwrap();
    let four = seg_hash.get(&4).unwrap();
    // let seven = seg_hash.get(&7).unwrap();
    let result = match segment.len() {
        2 => 1,
        3 => 7,
        4 => 4,
        5 => match set.difference(one).count() {
            3 => 3,
            _ => match set.difference(four).count() {
                3 => 2,
                _ => 5
            }
        },
        6 => match set.difference(one).count() {
            5 => 6,
            _ => match set.difference(four).count() {
                3 => 0,
                _ => 9
            },
        },
        7 => 8,
        _ => panic!("askdljaslkdj")
    };

    result
}

// 1 compared to 6 derives config of 1
// 1 compared to 7 derives config of 7
// since 4 contains 1, 

// Path::Top("abc"), Path::BottomRight("abc"), Path::TopRight("abc")
// vs
// Path::TopRight("ab"), Path::BottomRight("ab")

fn count_unique_digits(output: &Vec<String>) -> usize {
    let mut count = 0;
    for digit in output {
        match digit.len() {
            2 | 4 | 3 | 7 => count += 1,
            _ => {}
        }
    }

    count
}
// ex
// dgc gbfde aebdgf gecfd cd gecdbaf cfgedb dbgafc efagc decb | gebdf cd fcage dabcfeg
// 1 = CD or DC
// 7 = G | CD or CD
// 6 can define what 1 is through what it doesn't have in 7
// 3 of the 7 are defined here.
// then what 

fn day_nine() {
    let raw_input = read_to_string("./day9.txt")
        .unwrap();
    let mut grid: Vec<Vec<(u32, Point)>> = vec![];
    for (x, line) in raw_input.lines().enumerate() {
        let mut row: Vec<(u32, Point)> = vec![];
        for (y, n) in line.chars().enumerate() {
            row.push((n.to_digit(10).unwrap() as u32, Point(x, y)));
        }
        grid.push(row);
    }
    let mut depth_map = DepthMap::new(grid);

    depth_map.locate_lowest_points();
    let risk_level = depth_map.get_risk_level();
    let mut h: HashSet<(usize, usize)> = HashSet::new();
    h.insert((1, 2));
    h.insert((2,1));
    h.insert((2,1));
    // println!("risk level: {:?}", depth_map);
    depth_map.print_map();
    
    let mut sizes: Vec<usize> = vec![];
    for point in depth_map.lowest_points.clone().iter() {
        let mut visited: HashSet<Point> = HashSet::new();
        depth_map.search(*point, &mut visited);
        println!("{:?}", visited);
        let size = visited.len();
        sizes.push(size);
    }

    sizes.sort();
    sizes.reverse();
    let top_three = sizes[0] * sizes[1] * sizes[2];
    println!("{:?}", top_three);

}

struct DepthMap {
    pub grid: Vec<Vec<(u32, Point)>>,
    pub lowest_points: Vec<Point>,
    y_max: usize,
    x_max: usize
}

impl fmt::Debug for DepthMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let borders = self.get_basin_borders();
        let mut buffer = String::new();
        for x in 0..self.x_max {
            for y in 0..self.y_max {
                let (v, _) = self.grid[x][y];
                match v {
                    9 => buffer.push_str("#"),
                    _ => buffer.push_str(" ")
                }
            }
            buffer.push_str("\n");
        }
        f.debug_struct("DepthMap")
            .field("grid", &buffer)
            .finish()
        // f.debug_struct("DepthMap")
        //     .
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point(usize, usize);


impl DepthMap {
    pub fn new(grid: Vec<Vec<(u32, Point)>>) -> Self {
        let y_max = grid[0].len();
        let x_max = grid.len();
        DepthMap {
            grid: grid, lowest_points: vec![], x_max: x_max, y_max: y_max
        }
    }

    pub fn get_basin_borders(&self) {
        let mut basin_points: Vec<(usize, usize)> = vec![];

        for x in 0..self.x_max {
            for y in 0..self.y_max {
                let (v, _) = self.grid[x][y];
                if v == 9 {
                    basin_points.push((x, y));
                }
            }
        }

    }

    pub fn print_map(&self) {
        let mut buffer = String::new();
        for x in 0..self.x_max {
            for y in 0..self.y_max {
                let (v, _) = self.grid[x][y];
                match v {
                    9 => buffer.push_str("#"),
                    _ => {
                        let is_lowest_point = self.lowest_points.iter().find(|&p| p.0 == x && p.1 == y);
                        match is_lowest_point {
                            Some(p) => buffer.push_str("X"),
                            None => {
                                    buffer.push_str(".");
                            }
                        }
                    }
                }
            }
            buffer.push_str("\n");
        }
        println!("{}", buffer);
    }

    pub fn search(&mut self, start: Point, visited: &mut HashSet<Point>) {
        // this better work
        visited.insert(start);

        for neighbor in self.neighbors((start.0 as isize, start.1 as isize)) {
            match neighbor {
                Some((v, pt)) => {
                    if v < 9 && !visited.contains(&pt) {
                        self.search(pt, visited);
                    }
                },
                None => {}
            }
        }
    }

    fn neighbors(&self, point: (isize, isize)) -> Vec<Option<(u32, Point)>> {
        let mut neighbors = vec![];
        let (x, y) = point;
        let adjacent_points: [(isize, isize); 4] = [
            (x - 1, y),
            (x, y + 1),
            (x + 1, y),
            (x, y - 1)
        ];

        for pt in adjacent_points {
            neighbors.push(self.get(pt));
        }

        neighbors
    }

    pub fn get_risk_level(&self) -> u32 {
        self.lowest_points.iter().map(|Point(x,y)| {
            let (v, _) = self.grid[*x][*y];
            v + 1
        }).sum()
    }

    pub fn locate_lowest_points(&mut self) {
        for x in 0..self.x_max {
            for y in 0..self.y_max {
                let (v, _) = self.grid[x][y];
                if self.has_higher_neighbors((x as isize, y as isize), v) {
                    self.lowest_points.push(Point(x, y));
                }
            }
        }
    }

    fn get(&self, point: (isize, isize)) -> Option<(u32, Point)> {
        if point.0 < 0 || point.1 < 0 {
            return None
        }
        let (x, y) = point;

        match self.grid.get(x as usize) {
            Some(row) => {
                match row.get(y as usize) {
                    Some(n) => Some(*n),
                    None => None
                }
            },
            None => None
        }
    }

    fn has_higher_neighbors(&self, point: (isize, isize), value: u32) -> bool {
        let (x, y) = point;
        let adjacent_points: [(isize, isize); 4] = [
            (x - 1, y),
            (x, y + 1),
            (x + 1, y),
            (x, y - 1)
        ];
        let mut is_lowest_point = false;
        for point in adjacent_points {
            is_lowest_point = match self.get(point) {
                Some((n, _)) => n > value,
                None => true
            };
            if !is_lowest_point {
                return false
            }
        }
        is_lowest_point
    }
}

fn day_ten() {
    let input = read_to_string("./day10.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        total += count_errors(line);
    }
    println!("total p1: {}", total);
}

fn count_errors(line: &str) -> u32 {
    println!("{}", line);
    let mut chars: Vec<char> = line.chars().collect();
    let (mut squiglies, mut brackets, mut angle_brackets, mut parens) = (0, 0, 0, 0);
    for character in chars {
        match character {
            '[' | ']' => brackets += 1,
            '(' | ')' => parens += 1,
            '{' | '}' => squiglies += 1,
            '<' | '>' => angle_brackets += 1,
            _ => unimplemented!()
        }
    }
    let mut score = 0;
    if brackets > 0 && brackets % 2 != 0 {
        score += 57;
    } else if squiglies > 0 && squiglies % 2 != 0 {
        score += 1197;
    } else if angle_brackets > 0 && angle_brackets % 2 != 0 {
        score += 25137;
    } else if parens > 0 && parens % 2 != 0 {
        score += 3;
    }
    println!("[]: {}, {{}}: {}, (): {}, <>: {}, score: {}", brackets, squiglies, parens, angle_brackets, score);

    score
}
