use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn string_to_bool(input: &str) -> bool {
    match input {
        "1" => true,
        "0" => false,
        _ => panic!("Invalid input"),
    }
}

fn load_initial_state(file_path: &str) -> io::Result<HashMap<String, bool>> {
    let file = File::open(file_path).expect("file not found");
    let lines = io::BufReader::new(file).lines();
    let mut state = HashMap::new();
    for line in lines {
        let line = line?;
        let parts: Vec<&str> = line.split(": ").collect();
        state.insert(parts[0].to_string(), string_to_bool(parts[1]));
    }
    Ok(state)
}

fn load_circuit(file_path: &str) -> io::Result<HashMap<String, (String, String, String)>> {
    let file = File::open(file_path).expect("file not found");
    let lines = io::BufReader::new(file).lines();
    let mut circuit = HashMap::new();
    for line in lines {
        let line = line?;
        let parts: Vec<&str> = line.split(" -> ").collect();
        let inputs: Vec<&str> = parts[0].split(" ").collect();
        let tuple = (
            inputs[0].to_string(),
            inputs[1].to_string(),
            inputs[2].to_string(),
        );
        circuit.insert(parts[1].to_string(), tuple);
    }
    Ok(circuit)
}

fn check_key(key: &String, state: &HashMap<String, bool>) -> bool {
    if let Some(_) = state.get(key) {
        return true;
    }
    false
}

fn find_initial_inputs(
    wire: &String,
    circuit: &HashMap<String, (String, String, String)>,
) -> Vec<String> {
    let mut inputs = Vec::new();
    let (left, _, right) = circuit.get(wire).unwrap();
    if left.starts_with("x") || left.starts_with("y") {
        inputs.push(left.clone());
        return inputs;
    }
    if right.starts_with("x") || right.starts_with("y") {
        inputs.push(right.clone());
        return inputs;
    }
    let left_inputs = find_initial_inputs(left, circuit);
    let right_inputs = find_initial_inputs(right, circuit);
    inputs.extend(left_inputs);
    inputs.extend(right_inputs);
    inputs
}

fn eveluate_wire(
    wire: &String,
    circuit: &HashMap<String, (String, String, String)>,
    state: &HashMap<String, bool>,
) -> bool {
    // if state.contains_key(wire) {
    //     state[wire];
    // }
    if check_key(wire, state) {
        return state[wire];
    }
    // println!("Evaluating wire: {}", wire);
    let (left, op, right) = circuit.get(wire).unwrap();
    if op == "AND" {
        return eveluate_wire(left, circuit, state) && eveluate_wire(right, circuit, state);
    }
    if op == "OR" {
        return eveluate_wire(left, circuit, state) || eveluate_wire(right, circuit, state);
    }
    if op == "XOR" {
        return eveluate_wire(left, circuit, state) ^ eveluate_wire(right, circuit, state);
    }
    panic!("Invalid operator");
}

fn main() {
    // let state_file_path = "initial_state_small";
    let state_file_path = "initial_state";
    let state = load_initial_state(state_file_path).unwrap();
    // println!("{:?}", state);

    // let circuit_file_path = "circuit_small";
    let circuit_file_path = "circuit";
    let circuit = load_circuit(circuit_file_path).unwrap();
    // println!("{:?}", circuit);
    //
    // let start_inputs = find_initial_inputs(&"z12".to_string(), &circuit);
    // println!("{:?}", start_inputs);

    let z_wires: Vec<&String> = circuit.keys().filter(|key| key.starts_with("z")).collect();
    let mut outputs: Vec<bool> = vec![false; z_wires.len()];
    // println!("{:?}", outputs);

    for wire in z_wires {
        let index = wire
            .clone()
            .strip_prefix("z")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        // println!("Evaluating wire: {}", wire);
        let value = eveluate_wire(wire, &circuit, &state);
        outputs[index] = value;
        // println!("{}, index {}: {}", wire, index, value);
    }
    outputs.reverse();
    // println!("{:?}", outputs);
    let binary_number = outputs
        .iter()
        .fold(0, |acc, &bit| (acc << 1) | (bit as u64));

    println!("Result part 1: {}", binary_number);
}
