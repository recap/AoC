use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

// Read a file line by line
fn load_data(filename: &str) -> io::Result<HashMap<i64, Vec<i32>>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut data = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        if let Some((key, value)) = line.split_once(":") {
            let key = key.trim().parse::<i64>().unwrap();
            let values: Vec<i32> = value
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            // Check for duplicate keys
            if data.contains_key(&key) {
                println!("Key: {:?}, Values: {:?}", key, values);
            }
            data.insert(key, values);
        }
    }
    Ok(data)
}

fn generate_combinations_of_2(size: usize) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    let num_combinations = 1 << size; // 2^size combinations

    for i in 0..num_combinations {
        let mut combination = Vec::with_capacity(size);
        for j in (0..size).rev() {
            combination.push(((i >> j) & 1) as u8); // Extract each bit
        }
        result.push(combination);
    }

    result
}

fn generate_combinations_of_3(size: usize) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    let num_combinations = 3_usize.pow(size as u32); // 3^size combinations

    for i in 0..num_combinations {
        let mut combination = Vec::with_capacity(size);
        let mut value = i;

        for _ in 0..size {
            combination.push((value % 3) as u8); // Extract the remainder (0, 1, or 2)
            value /= 3; // Reduce the value for the next position
        }

        combination.reverse(); // Reverse to correct the order
        result.push(combination);
    }

    result
}

fn concatenate_as_i64(a: i64, b: i64) -> i64 {
    let concatenated = format!("{}{}", a, b); // Convert to strings and concatenate
    concatenated.parse::<i64>().unwrap() // Parse back to i64
}

fn find_ops(total: &i64, values: &Vec<i32>, p: u8) -> Vec<u8> {
    let mut ops: Vec<u8> = Vec::new();
    // let combinations = generate_combinations(values.len() - 1);
    let combinations = match p {
        2 => generate_combinations_of_2(values.len() - 1),
        3 => generate_combinations_of_3(values.len() - 1),
        _ => panic!("Invalid size"),
    };
    for combination in combinations {
        let mut combination_sub_total = values[0] as i64;
        for (i, op) in combination.iter().enumerate() {
            let iv = i + 1;
            let left = combination_sub_total;
            let right = values[iv] as i64;
            let result = match op {
                0 => left + right,
                1 => left * right,
                2 => concatenate_as_i64(left, right),
                _ => panic!("Invalid operation"),
            };
            combination_sub_total = result;
        }
        if combination_sub_total == *total {
            ops = combination.clone();
            return ops;
        }
    }
    ops
}

fn main() {
    let file_path = "input";
    let data = load_data(file_path).unwrap();
    let mut total: i64 = 0;
    for (key, values) in &data {
        let ops = find_ops(&key, &values, 2);
        if !ops.is_empty() {
            total += key;
            // println!("Key: {:?}, Values: {:?}, Ops: {:?}", key, values, ops);
        }
    }
    println!("Result part 1: {:?}", total);
    total = 0;
    for (key, values) in &data {
        let ops = find_ops(&key, &values, 3);
        if !ops.is_empty() {
            total += key;
            // println!("Key: {:?}, Values: {:?}, Ops: {:?}", key, values, ops);
        }
    }
    println!("Result part 2: {:?}", total);
    // println!("{:?}", data);
}
