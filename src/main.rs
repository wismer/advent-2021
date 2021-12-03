use std::fs::read_to_string;
use std::env::args;

fn main() {
    day_one_part_one();
    day_one_part_two();

    day_two_part_one();
    day_two_part_two();

    day_three_part_one();
    day_three_part_two();
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
    if !find_majority {

        println!("Data: {:?}", nonleading_bits);
    }
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