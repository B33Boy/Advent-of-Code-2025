use std::sync::OnceLock;
use std::{fs, io, path::Path};

static DIRS: &[(i32, i32)] = &[
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
    (0, -1),
    (-1, 0),
    (0, 1),
    (1, 0),
];
static ROWS: OnceLock<usize> = OnceLock::new();
static COLS: OnceLock<usize> = OnceLock::new();

fn main() -> io::Result<()> {
    println!("--- Day 4 Solution ---");

    let path = Path::new("input/day4.txt");
    let contents = fs::read_to_string(path).unwrap();

    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    ROWS.get_or_init(|| grid.len());
    COLS.get_or_init(|| grid[0].len());

    println!("--- Part 1 ---");
    println!("{}", q1(&grid));

    // println!("--- Part 2 ---");
    // println!("{}", q2(&data));

    Ok(())
}

fn get_rows() -> usize {
    *ROWS.get().expect("ROWS not initialized")
}

fn get_cols() -> usize {
    *COLS.get().expect("COLS not initialized")
}

fn q1(grid: &[Vec<char>]) -> usize {
    let mut rolls = 0;

    for row in 0..get_cols() {
        for col in 0..get_rows() {
            if can_access(grid, row, col) {
                rolls += 1;
            }
        }
    }

    rolls
}

fn can_access(grid: &[Vec<char>], row: usize, col: usize) -> bool {

    if grid[row][col] != '@' {
        return false;
    }

    let mut count = 0;

    for (x, y) in DIRS {
        let new_row: i32 = row as i32 + x;
        let new_col: i32 = col as i32 + y;

        // Check out of bounds
        if new_row < 0
            || new_row >= get_rows() as i32
            || new_col < 0
            || new_col >= get_cols() as i32
        {
            continue;
        }

        let new_row = new_row as usize;
        let new_col = new_col as usize;

        if grid[new_row][new_col] == '@' {
            count += 1;
        }
    }

    count < 4
}
