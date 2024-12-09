use std::fs::File;
use std::io::{self, BufRead};

fn load_data(file_path: &str) -> io::Result<Vec<u8>> {
    let file = File::open(file_path)?;
    let lines = io::BufReader::new(file).lines();
    let mut data: Vec<u8> = Vec::new();
    for line in lines {
        let line = line?;
        for c in line.chars() {
            data.push(c.to_digit(10).unwrap() as u8);
        }
    }
    if data.len() % 2 != 0 {
        data.push(0);
    }
    Ok(data)
}

fn find_next_minus_one(vec: &Vec<i32>, start_index: usize) -> Option<usize> {
    vec[start_index..]
        .iter()
        .position(|&x| x == -1)
        .map(|pos| pos + start_index)
}

fn find_next_block(vec: &[i32], start_index: usize, number: i32) -> Option<(usize, usize)> {
    let mut i = start_index;
    let len = vec.len();

    while i < len {
        if vec[i] == number {
            let block_start = i;
            let mut block_size = 0;

            // Count the size of the contiguous block
            while i < len && vec[i] == number {
                block_size += 1;
                i += 1;
            }

            return Some((block_start, block_size));
        }
        i += 1;
    }

    None
}

fn uncompress_data(data: &Vec<u8>) -> Vec<i32> {
    let mut uncompressed: Vec<i32> = Vec::new();
    let mut id_generator: i32 = 0;
    for k in (0..data.len()).step_by(2) {
        let d: u8 = data[k];
        let free: u8 = data[k + 1];
        for _ in 0u8..d {
            uncompressed.push(id_generator);
        }
        for _ in 0u8..free {
            uncompressed.push(-1);
        }
        id_generator += 1;
    }
    uncompressed
}

fn main() {
    // let file_path = "input_small";
    let file_path = "input";
    let data = load_data(file_path).unwrap();
    let mut uncompressed = uncompress_data(&data);

    // Part One
    let mut index = 0;
    for (p, c) in uncompressed.clone().iter().rev().enumerate() {
        let original_index = uncompressed.len() - p - 1;
        if *c == -1 {
            continue;
        }
        let next_index = find_next_minus_one(&uncompressed, index).unwrap();
        if original_index <= next_index {
            break;
        }
        uncompressed.swap(next_index, original_index);
        index = next_index;
    }

    // for c in uncompressed.iter() {
    //     if *c == -1 {
    //         print!(".");
    //         continue;
    //     }
    //     print!("{}|", c);
    // }
    // println!();

    let mut summation: i64 = 0;
    for (i, c) in uncompressed.iter().enumerate() {
        if *c == -1 {
            // print!(".");
            continue;
        }
        let m: i32 = i as i32 * *c;
        summation += m as i64;
        // print!("{}", c);
    }
    println!("Result Part 1: {}", summation);

    let mut uncompressed = uncompress_data(&data);
    let mut reversed = uncompressed.clone();
    reversed.reverse();
    let mut p = 0;
    while p < reversed.len() {
        let c = &reversed[p];
        if *c == -1 {
            p += 1;
            continue;
        }
        let (block_start, block_size) = find_next_block(&reversed, p, *c).unwrap();
        // let slice = &reversed[block_start..block_start + block_size];
        // println!("p: {}, slice: {:?}", p, slice);
        let original_block_start = uncompressed.len() - block_start - block_size;
        let mut free_block_start = 0;
        let mut free_block_size = 0;
        let mut search_index = 0;

        loop {
            match find_next_block(&uncompressed, search_index, -1) {
                Some(v) => {
                    free_block_start = v.0;
                    free_block_size = v.1;
                    if free_block_size >= block_size {
                        break;
                    }
                    search_index = free_block_start + free_block_size;
                }
                None => {
                    break;
                }
            }
        }
        // println!(
        //     "p: {}, free_block_start: {}, free_block_size: {}",
        //     p, free_block_start, free_block_size
        // );

        if free_block_size >= block_size {
            if original_block_start <= free_block_start {
                p = block_start + block_size;
                continue;
            }
            for s in 0..block_size {
                uncompressed.swap(original_block_start + s, free_block_start + s);
            }
        }
        p = block_start + block_size;
    }

    // for c in uncompressed.iter() {
    //     if *c == -1 {
    //         print!(".");
    //         continue;
    //     }
    //     print!("{}|", c);
    // }
    // println!();

    let mut summation: i64 = 0;
    for (i, c) in uncompressed.iter().enumerate() {
        if *c == -1 {
            // print!(".");
            continue;
        }
        let m: i32 = i as i32 * *c;
        summation += m as i64;
        // print!("{}", c);
    }
    println!("Result Part 2: {}", summation);
}
