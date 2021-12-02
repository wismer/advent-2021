use std::fs::read_to_string;
use std::env::args;

fn main() {
    day_one_part_one();
    day_one_part_two();

    day_two_part_one();
    day_two_part_two();
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
