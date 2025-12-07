use std::fs::File;
use std::io::{self, BufRead, BufReader};
fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<String>, io::Error>>()?;
    Ok(lines)
}
fn string_to_digits(s: &str) -> Option<Vec<u32>> {
    s.chars()
        .map(|c| c.to_digit(10)) // returns Option<u32>
        .collect() // returns None if any character is not a digit
}
fn unique_fast(v: &[u32]) -> Vec<u32> {
    let mut seen = [false; 256];
    let mut result = Vec::new();

    for &n in v {
        if !seen[n as usize] {
            seen[n as usize] = true;
            result.push(n);
        }
    }

    result
}
fn main() -> io::Result<()> {
    let lines = read_lines("data/input")?;
    // let lines = read_lines("data/input2")?;
    let mut result1: u64 = 0;
    let mut result2: u64 = 0;
    for line in &lines {
        match string_to_digits(line) {
            Some(digits) => {
                // println!("{:?}", digits);
                let unique_line = unique_fast(&digits);
                let mut sorted = unique_line.clone();
                sorted.sort();
                sorted.reverse();
                // println!("{:?}", unique_line);
                // println!("{:?}", sorted);
                let mut joltage = 0;
                for n0 in &sorted {
                    for n1 in &sorted {
                        let n0_index = digits.iter().position(|&x| x == *n0).unwrap();
                        // let n1_index = digits.iter().position(|&x| x == *n1).unwrap();
                        if let Some(n1_index) = digits.iter().rposition(|&n| n == *n1) {
                            if n0_index < n1_index {
                                let s = format!("{}{}", n0, n1);
                                joltage = s.parse::<u32>().unwrap();
                                result1 += joltage as u64;
                                // println!("Joltage: {}", joltage);
                                break;
                            }
                        } else {
                            continue;
                        }
                    }
                    if joltage != 0 {
                        break;
                    }
                }
                // println!("---");
            } // [1, 2, 3, 4, 5, 6]
            None => println!("Input contains non-digit characters"),
        }
    }
    println!("\nResult part one: {}", result1);
    for line in &lines {
        match string_to_digits(line) {
            Some(digits) => {
                // println!("{:?}", digits);
                let unique_line = unique_fast(&digits);
                let mut sorted = unique_line.clone();
                sorted.sort();
                sorted.reverse();
                // println!("{:?}", unique_line);
                // println!("{:?}", sorted);
                let mut working_digits = digits.clone();
                let mut joltage = vec![0u32; 12];

                for j in 0..12 {
                    for n in &sorted {
                        // println!("Trying n: {}", n);
                        let n0_index = match working_digits.iter().position(|&x| x == *n) {
                            Some(index) => index,
                            None => continue,
                        };
                        if n0_index + (11 - j) <= working_digits.len() - 1 {
                            // println!("Found n: {}", n);
                            joltage[j] = *n;
                            working_digits = working_digits.split_off(n0_index + 1);
                            // println!("Working digits: {:?}", working_digits);
                            break;
                        }
                    }
                }

                let joltage_str = joltage
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join("");

                let joltage_value = joltage_str.parse::<u64>().unwrap();
                result2 += joltage_value;

                // println!("Joltage: {:?}", joltage_str);
                // println!("---");
            } // [1, 2, 3, 4, 5, 6]
            None => println!("Input contains non-digit characters"),
        }
    }
    println!("\nResult part two: {}", result2);
    Ok(())
}
