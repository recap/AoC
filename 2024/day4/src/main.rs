use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn to_coordinates(&self) -> (i32, i32) {
        match self {
            Direction::N => (0, -1),
            Direction::NE => (1, -1),
            Direction::E => (1, 0),
            Direction::SE => (1, 1),
            Direction::S => (0, 1),
            Direction::SW => (-1, 1),
            Direction::W => (-1, 0),
            Direction::NW => (-1, -1),
        }
    }
}

struct Directions;

impl IntoIterator for Directions {
    type Item = Direction;
    type IntoIter = std::vec::IntoIter<Direction>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            Direction::N,
            Direction::NE,
            Direction::E,
            Direction::SE,
            Direction::S,
            Direction::SW,
            Direction::W,
            Direction::NW,
        ]
        .into_iter()
    }
}

fn load_file_to_matrix(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let matrix: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    Ok(matrix)
}

fn move_in_matrix(x: i32, y: i32, direction: &Direction) -> (i32, i32) {
    let (dx, dy) = direction.to_coordinates();
    (x + dx, y + dy)
}

fn get_matrix_size<T>(matrix: &[Vec<T>]) -> (usize, usize) {
    let rows = matrix.len();
    let cols = if rows > 0 { matrix[0].len() } else { 0 };
    (rows, cols)
}

fn is_out_of_bounds(current_pos: (i32, i32), matrix_size: (usize, usize)) -> bool {
    let (x, y) = current_pos;
    let (rows, cols) = matrix_size;
    x < 0 || y < 0 || x >= rows as i32 || y >= cols as i32
}

fn get_window(matrix: &[Vec<char>], start_pos: (i32, i32), direction: &Direction) -> Vec<char> {
    let mut current_pos = start_pos;
    let mut window = Vec::new();
    let matrix_size = get_matrix_size(matrix);
    for _ in 0..4 {
        let (x, y) = current_pos;
        if !is_out_of_bounds(current_pos, matrix_size) {
            let cell = matrix[y as usize][x as usize];
            window.push(cell);
            current_pos = move_in_matrix(x, y, direction);
        } else {
            window.push('F');
            current_pos = move_in_matrix(x, y, direction);
        }
    }
    window
}

fn get_forward_x(matrix: &[Vec<char>], start_pos: (i32, i32)) -> Vec<char> {
    let (x, y) = start_pos;
    let mut window = Vec::new();
    let matrix_size = get_matrix_size(matrix);
    let (se_x, se_y) = move_in_matrix(x, y, &Direction::SE);
    let (nw_x, nw_y) = move_in_matrix(x, y, &Direction::NW);
    if !is_out_of_bounds((se_x, se_y), matrix_size) {
        let cell = matrix[se_y as usize][se_x as usize];
        window.push(cell);
    } else {
        window.push('F');
    }
    let actual_cell = matrix[y as usize][x as usize];
    window.push(actual_cell);
    if !is_out_of_bounds((nw_x, nw_y), matrix_size) {
        let cell = matrix[nw_y as usize][nw_x as usize];
        window.push(cell);
    } else {
        window.push('F');
    }
    window
}
fn get_backword_x(matrix: &[Vec<char>], start_pos: (i32, i32)) -> Vec<char> {
    let (x, y) = start_pos;
    let mut window = Vec::new();
    let matrix_size = get_matrix_size(matrix);
    let (se_x, se_y) = move_in_matrix(x, y, &Direction::SW);
    let (nw_x, nw_y) = move_in_matrix(x, y, &Direction::NE);
    if !is_out_of_bounds((se_x, se_y), matrix_size) {
        let cell = matrix[se_y as usize][se_x as usize];
        window.push(cell);
    } else {
        window.push('F');
    }
    let actual_cell = matrix[y as usize][x as usize];
    window.push(actual_cell);
    if !is_out_of_bounds((nw_x, nw_y), matrix_size) {
        let cell = matrix[nw_y as usize][nw_x as usize];
        window.push(cell);
    } else {
        window.push('F');
    }
    window
}

fn main() -> io::Result<()> {
    let file_path = "src/input";
    let matrix = load_file_to_matrix(file_path)?;
    let mut counter = 0;

    for (r, row) in matrix.iter().enumerate() {
        for (c, _) in row.iter().enumerate() {
            let cur_pos = (r as i32, c as i32);
            for direction in Directions {
                let window = get_window(&matrix, cur_pos, &direction);
                let s = window.iter().map(|c| c.to_string()).collect::<String>();
                if s == "XMAS" {
                    counter += 1;
                }
            }
        }
    }

    println!("Part 1: {}", counter);

    counter = 0;

    for (r, row) in matrix.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if cell != &'A' {
                continue;
            }
            let cur_pos = (c as i32, r as i32);
            let window_f = get_forward_x(&matrix, cur_pos);
            let window_b = get_backword_x(&matrix, cur_pos);
            let f = window_f.iter().map(|c| c.to_string()).collect::<String>();
            let b = window_b.iter().map(|c| c.to_string()).collect::<String>();
            if (f == "MAS" || f == "SAM") && (b == "MAS" || b == "SAM") {
                counter += 1;
            }
        }
    }
    println!("Part 2: {}", counter);

    Ok(())
}
