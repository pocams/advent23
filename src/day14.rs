use std::collections::HashMap;

use color_eyre::Report;
use tracing::{debug, info};

const ROUND_ROCK: u8 = b'O';
#[allow(dead_code)]
const SQUARE_ROCK: u8 = b'#';
const GROUND: u8 = b'.';

fn part1_load(grid: &[Vec<u8>]) -> i64 {
    let mut load = 0;
    for (idx, row) in grid.iter().enumerate() {
        let rock_value = grid.len() - idx;
        load += rock_value * row.iter().filter(|c| **c == ROUND_ROCK).count()
    }
    load as i64
}

// I was going to do a rotate then roll, but I figured I'd wait until p2 to see whether the
// roll operation had to be optimized.  Now it doesn't seem worth changing.

fn roll_north(grid: &mut Vec<Vec<u8>>) {
    for col_num in 0..grid[0].len() {
        for mut row_num in 0..grid.len() {
            if grid[row_num][col_num] == ROUND_ROCK {
                while row_num > 0 && grid[row_num - 1][col_num] == GROUND {
                    row_num -= 1;
                    debug!(row=row_num, col=col_num, "up");
                    grid[row_num][col_num] = ROUND_ROCK;
                    grid[row_num + 1][col_num] = GROUND;
                }
            }
        }
    }
}

fn roll_south(grid: &mut Vec<Vec<u8>>) {
    for col_num in 0..grid[0].len() {
        for mut row_num in (0..grid.len()).rev() {
            if grid[row_num][col_num] == ROUND_ROCK {
                while row_num < grid.len() - 1 && grid[row_num + 1][col_num] == GROUND {
                    row_num += 1;
                    debug!(row=row_num, col=col_num, "down");
                    grid[row_num][col_num] = ROUND_ROCK;
                    grid[row_num - 1][col_num] = GROUND;
                }
            }
        }
    }
}

fn roll_west(grid: &mut Vec<Vec<u8>>) {
    for row_num in 0..grid.len() {
        for mut col_num in 0..grid[0].len() {
            if grid[row_num][col_num] == ROUND_ROCK {
                while col_num > 0 && grid[row_num][col_num - 1] == GROUND {
                    col_num -= 1;
                    debug!(row=row_num, col=col_num, "left");
                    grid[row_num][col_num] = ROUND_ROCK;
                    grid[row_num][col_num + 1] = GROUND;
                }
            }
        }
    }
}

fn roll_east(grid: &mut Vec<Vec<u8>>) {
    for row_num in 0..grid.len() {
        for mut col_num in (0..grid[0].len()).rev() {
            if grid[row_num][col_num] == ROUND_ROCK {
                while col_num < grid[0].len() - 1 && grid[row_num][col_num + 1] == GROUND {
                    col_num += 1;
                    debug!(row=row_num, col=col_num, "right");
                    grid[row_num][col_num] = ROUND_ROCK;
                    grid[row_num][col_num - 1] = GROUND;
                }
            }
        }
    }
}

fn cycle(grid: &mut Vec<Vec<u8>>) {
    roll_north(grid);
    roll_west(grid);
    roll_south(grid);
    roll_east(grid);
}

fn show(grid: &[Vec<u8>]) {
    for row in grid {
        for col in row {
            print!("{}", *col as char);
        }
        println!();
    }
}

pub(crate) fn solve(input: String) -> Result<(), Report> {

    let mut grid: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    show(&grid);
    println!();
    roll_north(&mut grid);
    show(&grid);

    let load = part1_load(&grid);
    info!(day=14, part=1, answer=load);

    let mut seen_grids = HashMap::new();

    let target = 1000000000;

    for iter in 1..100000 {
        cycle(&mut grid);
        let mut state: Vec<u8> = Vec::with_capacity(grid.len() * grid[0].len());
        for row in grid.iter() { state.extend_from_slice(row) };
        let seen = seen_grids.get(&state);
        if let Some(s) = seen {
            let cycle = iter - s;
            let load = part1_load(&grid);
            info!("duplicate: {} = {} (cycle {}) load={}", iter, s, cycle, load);
            if (iter - s) % cycle == (target - s) % cycle {
                info!(day=14, part=2, answer=load);
                break;
            }
        } else {
            seen_grids.insert(state, iter);
        }
    }

    Ok(())
}
