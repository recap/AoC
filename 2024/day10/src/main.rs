use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn to_coordinates(&self) -> (i32, i32) {
        match self {
            Direction::UP => (0, -1),
            Direction::DOWN => (0, 1),
            Direction::LEFT => (-1, 0),
            Direction::RIGHT => (1, 0),
        }
    }
}

struct Directions;

impl IntoIterator for Directions {
    type Item = Direction;
    type IntoIter = std::vec::IntoIter<Direction>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            Direction::UP,
            Direction::DOWN,
            Direction::LEFT,
            Direction::RIGHT,
        ]
        .into_iter()
    }
}

fn load_data(file_path: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(file_path)?;
    let lines = io::BufReader::new(file).lines();
    let mut data = Vec::new();
    for line in lines {
        let line = line?;
        let row: Vec<i32> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        data.push(row);
    }
    Ok(data)
}

fn find_trail_heads(data: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut trail_heads = Vec::new();
    for (y, row) in data.iter().enumerate() {
        for (x, &d) in row.iter().enumerate() {
            if d == 0 {
                trail_heads.push((x, y));
            }
        }
    }
    trail_heads
}

fn walk(data: &Vec<Vec<i32>>, pos: (usize, usize)) -> HashSet<Vec<(usize, usize)>> {
    // Collect data during recursion
    let mut trails = HashSet::new();
    let mut points = Vec::new();

    // Recursive function
    fn _walk(
        data: &Vec<Vec<i32>>,
        pos: (usize, usize),
        points: &mut Vec<(usize, usize)>,
        trails: &mut HashSet<Vec<(usize, usize)>>,
    ) {
        let current = data[pos.1][pos.0];
        // Try all directions
        for d in Directions {
            let (dx, dy) = d.to_coordinates();
            let x = pos.0 as i32 + dx;
            let y = pos.1 as i32 + dy;
            // Check if out of bounds
            if x < 0 || x >= data[0].len() as i32 || y < 0 || y >= data.len() as i32 {
                continue;
            }
            let d_pos = data[y as usize][x as usize];
            // Check if next step is next in trail
            if d_pos == (current + 1) {
                points.push((x as usize, y as usize));
                // Cech if end of trail
                if d_pos == 9 {
                    // println!("Found end of trail {} at ({}, {})", d_pos, x, y);
                    // println!("Points: {:?}", points);
                    // Add trail to set and clear points for next trail
                    trails.insert(points.clone());
                    // points.clear();
                } else {
                    // If not end of trail, continue recursion
                    _walk(data, (x as usize, y as usize), points, trails);
                }
            }
        }
    }
    // Start recursion
    _walk(data, pos, &mut points, &mut trails);
    // Trails is mutated during recursion
    trails
}

fn main() {
    // let file_path = "input_small";
    let file_path = "input";
    let data = load_data(file_path).unwrap();
    let mut total_sum = 0;
    let trail_heads = find_trail_heads(&data);

    // Question 1
    for (x, y) in &trail_heads {
        // println!("Trail head at ({}, {})", x, y);
        let trails = walk(&data, (*x, *y));
        let mut unique_ends = HashSet::new();

        for path in trails {
            // println!("Trail: {:?}", path);
            let end = path.last().unwrap();
            unique_ends.insert(end.clone());
        }
        let s = unique_ends.len();
        total_sum += s;
        // println!("Trail Score: {}", s);
    }
    println!("Question 1 Total Score: {}", total_sum);

    // Question 2
    let mut total_sum = 0;
    for (x, y) in &trail_heads {
        let trails = walk(&data, (*x, *y));
        total_sum += &trails.len();
    }
    println!("Question 2 Total Score: {}", total_sum);
}
