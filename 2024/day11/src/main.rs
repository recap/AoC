use std::collections::HashMap;

fn is_even(n: usize) -> bool {
    n % 2 == 0
}

fn default_str_if_empty(s: String) -> String {
    if s.is_empty() {
        "0".to_string()
    } else {
        s
    }
}

fn split_in_two(s: String) -> (String, String) {
    let n = s.len();
    let half = n / 2;
    let first = s[0..half].trim_start_matches('0').to_string();
    let second = s[half..n].trim_start_matches('0').to_string();
    (default_str_if_empty(first), default_str_if_empty(second))
}

fn compute_iterations(s: &str, depth: u32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let i_s: String = s.to_string();
    let mut numbers: Vec<String> = i_s.split_whitespace().map(|i_s| i_s.to_string()).collect();
    for _ in 0..depth {
        result.clear();
        for d in &numbers {
            if d == " " {
                continue;
            }
            if d == "0" {
                result.push("1".to_string());
                continue;
            }
            if is_even(d.len()) {
                let (first, second) = split_in_two(d.to_string());
                result.push(first);
                result.push(second);
                continue;
            }
            let x = (d.parse::<i64>().unwrap() * 2024).to_string();
            result.push(x);
        }
        numbers = result.clone();
    }
    result
}

fn main() {
    let input_string = "2 72 8949 0 981038 86311 246 7636740";
    // let input_string = "125 17";
    let data: Vec<String> = input_string
        .split_whitespace()
        .map(|i_s| i_s.to_string())
        .collect();
    let intermediate_depth = 42;
    let iterations = 75;
    let mut total_count = 0;
    let mut map: HashMap<String, u64> = HashMap::new();
    for p in &data {
        let sub_part = compute_iterations(p, intermediate_depth as u32);

        println!(
            " Sub string {:?} Iteration {} length: {}",
            p,
            intermediate_depth,
            sub_part.len()
        );

        let mut count = 0;
        for d in &sub_part {
            if let Some(value) = map.get(d) {
                // println!("Cache hit! {:?}", d);
                count += value;
            } else {
                let computed = compute_iterations(d, iterations - intermediate_depth);
                map.insert(d.clone(), computed.len() as u64);
                count += computed.len() as u64;
            }
        }
        println!(
            " Sub string {:?} Iteration {} length: {}",
            p, iterations, count
        );
        total_count += count;
    }
    println!("Total count: {}", total_count);
}
