use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn read_columns(file_path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    let file = File::open(file_path)?;
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        if let (Some(part1), Some(part2)) = (parts.next(), parts.next()) {
            col1.push(part1.parse().unwrap());
            col2.push(part2.parse().unwrap());
        }
    }
    col1.sort();
    col2.sort();

    Ok((col1, col2))
}

fn subtract_vectors(vec1: &[i32], vec2: &[i32]) -> Vec<i32> {
    vec1.iter()
        .zip(vec2.iter())
        .map(|(a, b)| (a - b).abs())
        .collect()
}

fn histogram(vec: &[i32]) -> HashMap<i32, usize> {
    let mut hist = HashMap::new();
    for &val in vec {
        *hist.entry(val).or_insert(0) += 1;
    }
    hist
}

fn main() {
    let (col1, col2) = read_columns("src/input").unwrap();

    let result = subtract_vectors(&col1, &col2);
    let total: i32 = result.iter().sum();
    println!("Part 1 {:?}", total);

    let hist = histogram(&col2);
    let mut acc = 0;
    for v in col1.iter() {
        let f = *hist.get(&v).unwrap_or(&0) as i32;
        let s = v * f;
        acc += s;
    }

    println!("Part 2 {:?}", acc);
}
