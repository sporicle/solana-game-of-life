use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let mut matrix: [[bool; 10]; 10] = [[false; 10]; 10];
    matrix[3][3] = true;
    matrix[3][4] = true;
    matrix[3][5] = true;
    matrix[3][7] = true;
    matrix[4][3] = true;
    matrix[7][3] = true;
    matrix[6][4] = true;
    matrix[6][5] = true;
    matrix[6][7] = true;
    matrix[5][6] = true;
    matrix[5][7] = true;
    matrix[7][5] = true;
    matrix[7][7] = true;
    pretty_print(&matrix);

    for _ in 0..5 {
        clear_console();
        pretty_print(&matrix);
        matrix = update_state(&matrix);
        thread::sleep(Duration::from_secs(1));
    }
}

fn update_state(grid: &[[bool; 10]; 10]) -> [[bool; 10]; 10] {
    let mut new_grid = grid.clone();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let mut count = 0;
            for x in 0..3 {
                for y in 0..3 {
                    if i + x < 1 || i + x >= grid.len() + 1 || j + y < 1 || j + y >= grid[i].len() + 1 {
                        continue;
                    }
                    if grid[i + x - 1][j + y - 1] {
                        count += 1;
                    }
                }
            }
            match grid[i][j] {
                true => {
                    if count < 2 || count > 3 {
                        new_grid[i][j] = false;
                    }
                }
                false => {
                    if count == 3 {
                        new_grid[i][j] = true;
                    }
                }
            }
        }
    }
    new_grid
}

fn pretty_print(grid: &[[bool; 10]; 10]) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            match grid[i][j] {
                true => print!("* "),
                false => print!("- "),
            }
        }
        println!();
    }
    println!();
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}