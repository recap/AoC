use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn to_coordinates(&self) -> (i32, i32) {
        match self {
            Direction::N => (0, -1),
            Direction::E => (1, 0),
            Direction::S => (0, 1),
            Direction::W => (-1, 0),
        }
    }
}

fn move_in_matrix(x: i32, y: i32, direction: &Direction) -> (i32, i32) {
    let (dx, dy) = direction.to_coordinates();
    (x + dx, y + dy)
}

fn load_data_to_matrix(file_path: &str) -> Vec<Vec<char>> {
    let file = File::open(file_path).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut matrix = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let row = line.split_whitespace().flat_map(|x| x.chars()).collect();
        matrix.push(row);
    }
    matrix
}

fn find_guard(matrix: &Vec<Vec<char>>) -> (usize, usize) {
    let guard: Vec<char> = ['^', 'v', '<', '>'].to_vec();
    for (y, row) in matrix.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if guard.contains(&c) {
                return (x, y);
            }
        }
    }
    panic!("Guard not found");
}

fn rotate_direction_90deg(direction: &Direction) -> Direction {
    let next_direction: Direction = match direction {
        Direction::N => Direction::E,
        Direction::E => Direction::S,
        Direction::S => Direction::W,
        Direction::W => Direction::N,
    };

    next_direction
}

fn is_out_of_bounds(current_pos: (i32, i32), matrix_size: (usize, usize)) -> bool {
    let (x, y) = current_pos;
    let (rows, cols) = matrix_size;
    x < 0 || y < 0 || x >= rows as i32 || y >= cols as i32
}

fn count_x(matrix: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for row in matrix {
        for c in row {
            if *c == 'X' {
                count += 1;
            }
        }
    }
    count
}

fn check_for_loop(
    matrix: &Vec<Vec<char>>,
    pos: (usize, usize),
    obs_pos: (usize, usize),
    max_steps: i32,
) -> bool {
    let mut new_matrix = matrix.clone();
    let mut x = pos.0 as i32;
    let mut y = pos.1 as i32;
    let guard = matrix[y as usize][x as usize];
    let current_direction: Direction = match guard {
        '^' => Direction::N,
        'v' => Direction::S,
        '<' => Direction::W,
        '>' => Direction::E,
        _ => panic!("Invalid guard"),
    };
    new_matrix[obs_pos.1][obs_pos.0] = '#';
    let mut next_direction = current_direction;
    let mut loop_detected = true;
    let mut count = 0;
    loop {
        count += 1;
        let (nx, ny) = move_in_matrix(x, y, &next_direction);
        if is_out_of_bounds((nx, ny), (matrix[0].len(), matrix.len())) {
            loop_detected = false;
            break;
        }
        if count >= max_steps {
            break;
        }
        let cell = new_matrix[ny as usize][nx as usize];
        if cell == '.' || cell == 'X' {
            new_matrix[y as usize][x as usize] = 'X';
            x = nx;
            y = ny;
            continue;
        }
        if cell == '#' {
            next_direction = rotate_direction_90deg(&next_direction);
            continue;
        }
    }
    loop_detected
}

fn walk(matrix: &Vec<Vec<char>>, pos: (usize, usize)) -> Vec<Vec<char>> {
    let mut new_matrix = matrix.clone();
    let mut x = pos.0 as i32;
    let mut y = pos.1 as i32;
    let guard = matrix[y as usize][x as usize];
    let current_direction: Direction = match guard {
        '^' => Direction::N,
        'v' => Direction::S,
        '<' => Direction::W,
        '>' => Direction::E,
        _ => panic!("Invalid guard"),
    };
    let mut next_direction = current_direction;
    loop {
        let (nx, ny) = move_in_matrix(x, y, &next_direction);
        if is_out_of_bounds((nx, ny), (matrix[0].len(), matrix.len())) {
            break;
        }
        let cell = new_matrix[ny as usize][nx as usize];
        if cell == '.' || cell == 'X' {
            new_matrix[y as usize][x as usize] = 'X';
            x = nx;
            y = ny;
            continue;
        }
        if cell == '#' {
            next_direction = rotate_direction_90deg(&next_direction);
            continue;
        }
    }
    new_matrix
}

fn get_matrix_size<T>(matrix: &[Vec<T>]) -> (usize, usize) {
    let rows = matrix.len();
    let cols = if rows > 0 { matrix[0].len() } else { 0 };
    (rows, cols)
}

fn main() {
    let file_path = "input";
    let matrix = load_data_to_matrix(file_path);
    let pos = find_guard(&matrix);
    let guard = matrix[pos.1][pos.0];
    println!("Guard found at position: {:?}, direction {}", pos, guard);
    let new_matrix = walk(&matrix, pos);
    let count = count_x(&new_matrix);
    println!("Count: {}", count + 1);

    let mut loop_counter = 0;
    let mat_size = get_matrix_size(&matrix);
    println!("Matrix size: {:?}", mat_size);
    for x in 0..mat_size.0 {
        for y in 0..mat_size.1 {
            // println!("Checking position: {:?}", (x, y));
            if matrix[y][x] == '.' {
                let is_looping = check_for_loop(&matrix, pos, (x, y), 10000);
                if is_looping {
                    loop_counter += 1;
                    // println!("Looping at position {:?}: {}", (x, y), is_looping);
                }
            }
        }
    }
    println!("Loop counter: {}", loop_counter);
}
