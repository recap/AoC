use std::fs::File;
use std::io::{self, BufRead, BufReader};
fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<String>, io::Error>>()?;
    Ok(lines)
}
fn parse_lr(input: &str) -> Option<i32> {
    let (dir, num) = input.split_at(1);
    let value: i32 = num.parse().ok()?;
    match dir {
        "L" => Some(-value),
        "R" => Some(value),
        _ => None,
    }
}
fn opposite_signs(a: i32, b: i32) -> bool {
    println!("Checking opposite signs for {} and {}", a, b);
    (a ^ b) < 0
}
fn main() -> io::Result<()> {
    let start: i32 = 50;
    let mut counter: u32 = 0;
    let mut zero_crossings: u32 = 0;
    let mut position: i32 = start;
    let lines = read_lines("data/input")?;
    let values: Vec<i32> = lines.iter().filter_map(|line| parse_lr(line)).collect();

    for value in values {
        let full_rotations = value.abs() / 100;
        let remainder = value % 100;
        let new_position = (position + value).rem_euclid(100);
        let crossed = if position != 0 && remainder > 0 && new_position != 0 {
            (new_position < position) as i32
        } else if position != 0 && new_position != 0 && remainder < 0 {
            (new_position > position) as i32
        } else {
            0
        };
        zero_crossings += full_rotations.abs() as u32 + crossed as u32;

        if new_position == 0 && remainder != 0 {
            counter += 1;
        }
        println!(
            "Current position: {}, moving by: {}, new position: {}, full rotations: {}, remainder: {}, crossed zero: {}, counter: {}",
            position, value, new_position, full_rotations, remainder, crossed, counter
        );

        position = new_position;
    }
    println!("Final position: {}", position);
    println!("Counter: {}", counter);
    println!("Zero crossings: {}", zero_crossings);
    println!("Total Zeros: {}", counter + zero_crossings);
    Ok(())
}
