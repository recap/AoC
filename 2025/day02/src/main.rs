use std::fs;
use std::io::{self};
fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let contents = fs::read_to_string(path)?;
    let items: Vec<String> = contents.split(',').map(|s| s.trim().to_string()).collect();
    Ok(items)
}
fn create_range(line: &str) -> io::Result<Vec<u64>> {
    // println!("Creating range from line: {}", line);
    let parts: Vec<&str> = line.split('-').collect();
    if parts.len() != 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid range format",
        ));
    }
    let start: u64 = parts[0]
        .trim()
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid start number"))?;
    let end: u64 = parts[1]
        .trim()
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid end number"))?;
    if start > end {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Start number greater than end number",
        ));
    }
    Ok((start..=end).collect())
}
fn is_repeated(num: u64) -> bool {
    let num_str = num.to_string();
    if num_str.len() % 2 != 0 {
        return false;
    }
    let mid = num_str.len() / 2;
    let (left, right) = num_str.split_at(mid);
    left == right
}
fn all_chars_same(s: &str) -> bool {
    if let Some(first) = s.chars().next() {
        s.chars().all(|c| c == first)
    } else {
        true // empty string → considered "all same"
    }
}
fn all_equal<T: PartialEq>(v: &[T]) -> bool {
    if let Some(first) = v.first() {
        v.iter().all(|x| x == first)
    } else {
        true // empty vector → considered "all equal"
    }
}
fn pass_check2(num: u64) -> bool {
    let s = num.to_string();
    for d in 2..=s.len() {
        let chunk_size = s.len() / d;
        let parts: Vec<String> = s
            .chars()
            .collect::<Vec<_>>() // collect into char vector
            .chunks(chunk_size) // take chunks
            .map(|c| c.iter().collect()) // turn each chunk back into String
            .collect();
        if all_equal(&parts) {
            return true;
        }
    }
    false
}
fn is_odd_len(num: u64) -> bool {
    let num_str = num.to_string();
    num_str.len() % 2 != 0
}
fn main() -> io::Result<()> {
    let lines = read_lines("data/input")?;
    // let lines = read_lines("data/input2")?;
    let mut result: u64 = 0;
    for line in &lines {
        let range = create_range(line)?;
        for num in &range {
            if is_repeated(*num) {
                result += num;
            }
        }
        // println!("Range for '{}': {:?}", line, range);
    }
    // println!("Lines: {:?}", lines);
    println!("\nResult part one: {}", result);
    let mut result2: u64 = 0;
    for line in &lines {
        let range = create_range(line)?;
        for num in &range {
            if pass_check2(*num) {
                println!("{}", num);
                result2 += num;
            }
        }
        // println!("Range for '{}': {:?}", line, range);
    }
    // println!("Lines: {:?}", lines);
    println!("\nResult part two: {}", result2);
    Ok(())
}
