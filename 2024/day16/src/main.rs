use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

// #[derive(Debug, Clone)]
#[derive(Ord, PartialOrd, Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_coordinates(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
    fn to_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    fn to_anticlockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}

fn load_file(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(file_path)?;
    let lines = io::BufReader::new(file).lines();
    let mut map = Vec::new();
    for line in lines {
        let line = line?;
        map.push(line.chars().collect());
    }
    Ok(map)
}

fn find_start(map: &Vec<Vec<char>>) -> (i32, i32) {
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                return (x as i32, y as i32);
            }
        }
    }
    panic!("No start found");
}

fn find_end(map: &Vec<Vec<char>>) -> (i32, i32) {
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'E' {
                return (x as i32, y as i32);
            }
        }
    }
    panic!("No start found");
}

fn manhattan_distance(a: (i32, i32), b: (i32, i32)) -> u32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32
}

fn is_valid(pos: (i32, i32), map: &Vec<Vec<char>>, path: &Vec<(i32, i32)>) -> bool {
    let (x, y) = pos;
    x >= 0
        && y >= 0
        && y < map.len() as i32
        && x < map[0].len() as i32
        && map[y as usize][x as usize] != '#'
    // && !path.contains(&pos)
}

fn a_star_search_with_paths(
    map: &Vec<Vec<char>>,
    start: (i32, i32),
    end: (i32, i32),
    best_score: u32,
) -> Option<(u32, Vec<Vec<(i32, i32)>>)> {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut min_score = u32::MAX;
    let mut best_paths = Vec::new();
    let mut loop_counter = HashMap::new();

    // Push initial state: (score, cost, position, direction)
    heap.push(Reverse((0, 0, start, Direction::Right, vec![start])));

    while let Some(Reverse((_score, cost, (x, y), direction, path))) = heap.pop() {
        // Check if we've reached the target symbol 'E'
        if map[y as usize][x as usize] == 'E' {
            if cost < min_score {
                min_score = cost;
                best_paths.clear();
                best_paths.push(path.clone());
            } else {
                if cost == min_score {
                    best_paths.push(path.clone());
                }
            }
            continue;
        }

        if cost > best_score {
            continue;
        }

        // Skip already visited states
        if !visited.insert(((x, y), direction)) {
            let counter = loop_counter.entry(((x, y), direction)).or_insert(20);
            *counter -= 1;
            if *counter <= 0 {
                continue;
            }
        }

        // Move in the current direction
        let (dx, dy) = direction.to_coordinates();
        let next = (x + dx, y + dy);
        if is_valid(next, map, &path) {
            let mut next_path = path.clone();
            next_path.push(next);
            heap.push(Reverse((cost, cost + 1, next, direction, next_path)));
        }

        // Turn clockwise
        let clockwise_dir = direction.to_clockwise();
        let (dx, dy) = clockwise_dir.to_coordinates();
        let next = (x + dx, y + dy);
        if is_valid(next, map, &path) {
            let mut next_path = path.clone();
            next_path.push(next);
            heap.push(Reverse((
                cost,
                cost + 1000 + 1,
                next,
                clockwise_dir,
                next_path,
            )));
        }

        // Turn anticlockwise
        let anticlockwise_dir = direction.to_anticlockwise();
        let (dx, dy) = anticlockwise_dir.to_coordinates();
        let next = (x + dx, y + dy);
        if is_valid(next, map, &path) {
            let mut next_path = path.clone();
            next_path.push(next);
            heap.push(Reverse((
                cost,
                cost + 1000 + 1,
                next,
                anticlockwise_dir,
                next_path,
            )));
        }
    }
    if !best_paths.is_empty() {
        Some((min_score, best_paths))
    } else {
        None
    }
}

fn a_star_search(map: &Vec<Vec<char>>, start: (i32, i32), end: (i32, i32)) -> Option<u32> {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    // Push initial state: (score, cost, position, direction)
    heap.push(Reverse((0, 0, start, Direction::Right)));

    while let Some(Reverse((_score, cost, (x, y), direction))) = heap.pop() {
        // Check if we've reached the target symbol 'E'
        if map[y as usize][x as usize] == 'E' {
            return Some(cost);
        }

        // Skip already visited states
        if !visited.insert(((x, y), direction)) {
            continue;
        }

        // Move in the current direction
        let (dx, dy) = direction.to_coordinates();
        let next = (x + dx, y + dy);
        if is_valid(next, map, &vec![]) {
            heap.push(Reverse((
                cost + 1 + manhattan_distance(next, end),
                cost + 1,
                next,
                direction,
            )));
        }

        // Turn clockwise
        let clockwise_dir = direction.to_clockwise();
        let (dx, dy) = clockwise_dir.to_coordinates();
        let next = (x + dx, y + dy);
        if is_valid(next, map, &vec![]) {
            heap.push(Reverse((
                cost + 1000 + 1 + manhattan_distance(next, end),
                cost + 1000 + 1,
                next,
                clockwise_dir,
            )));
        }

        // Turn anticlockwise
        let anticlockwise_dir = direction.to_anticlockwise();
        let (dx, dy) = anticlockwise_dir.to_coordinates();
        let next = (x + dx, y + dy);
        if is_valid(next, map, &vec![]) {
            heap.push(Reverse((
                cost + 1000 + 1 + manhattan_distance(next, end),
                cost + 1000 + 1,
                next,
                anticlockwise_dir,
            )));
        }
    }

    None
}

fn main() {
    let file_path = "input";
    // let file_path = "input_small";
    let mut map = load_file(file_path).unwrap();
    let start = find_start(&map);
    let end = find_end(&map);
    if let Some(score) = a_star_search(&map, start, end) {
        let mut set: HashSet<(i32, i32)> = HashSet::new();
        println!("Path found with score: {}", score);
        if let Some((min_score, best_paths)) = a_star_search_with_paths(&map, start, end, score) {
            println!("Minimum score: {}", min_score);
            // println!("Best paths:");
            for path in best_paths {
                set.extend(&path);
                // println!("{:?}", path);
                // println!();
            }
        } else {
            println!("No path found.");
        }

        // for point in &set {
        //     let (x, y) = point;
        //     map[*y as usize][*x as usize] = 'O';
        // }
        //
        // for row in &map {
        //     for cell in row {
        //         print!("{}", cell);
        //     }
        //     println!();
        // }

        println!("Unique positions: {}", set.len());
    } else {
        println!("No path found.");
    }
}
