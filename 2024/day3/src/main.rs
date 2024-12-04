use regex::Regex;
use std::fs;
use std::io;

fn read_file_to_string(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path)
}

fn extract_mul_pattern(input: &str) -> Vec<String> {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    re.find_iter(input)
        .map(|m| m.as_str().to_string())
        .collect()
}

fn calculate_mul_expressions(input: &str) -> Vec<i64> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let num1: i64 = cap[1].parse().unwrap();
            let num2: i64 = cap[2].parse().unwrap();
            num1 * num2
        })
        .collect()
}

fn remove_newlines(input: &str) -> String {
    let re = Regex::new(r"\n").unwrap();
    re.replace_all(input, "").to_string()
}

fn filter_out_bound_substrings(input: &str) -> String {
    let re = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    let int_string: String = re.replace_all(input, "").to_string();
    let re2 = Regex::new(r"don't\(\).*").unwrap();
    re2.replace_all(&int_string, "").to_string()
}

fn main() -> io::Result<()> {
    let file_content = read_file_to_string("src/input")?;
    let file_content = remove_newlines(&file_content);
    let results = calculate_mul_expressions(&file_content);
    let total: i64 = results.iter().sum();
    println!("Part 1: {}", total);
    let filtered_content = filter_out_bound_substrings(&file_content);
    let results = calculate_mul_expressions(&filtered_content);
    let total: i64 = results.iter().sum();
    println!("Part 2: {}", total);
    Ok(())
}
