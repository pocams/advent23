use std::ops::BitXor;
use color_eyre::Report;
use tracing::{debug, info};

fn row_to_u64(row: &[u8]) -> u64 {
    let mut value = 0;
    for &c in row {
        value <<= 1;
        if c == b'#' {
            value |= 1;
        }
    }
    value
}

fn rotate_grid(rows: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut out = vec![Vec::new(); rows[0].len()];
    for row in rows {
        for (idx, col) in row.iter().enumerate() {
            out[idx].push(*col);
        }
    }
    out
}

fn find_reflections(grid: &[u64]) -> Vec<usize> {
    let mut reflections = Vec::new();
    for i in 1..grid.len() {
        let mut left = i - 1;
        let mut right = i;
        loop {
            if grid[left] != grid[right] { break }
            if left == 0 || right == grid.len() - 1 {
                reflections.push(i);
                break;
            }
            left -= 1;
            right += 1;
        }
    }
    reflections
}

fn find_reflections_pt2(grid: &[u64]) -> Vec<usize> {
    let mut reflections = Vec::new();
    for i in 1..grid.len() {
        let mut left = i - 1;
        let mut right = i;
        let mut smudges = 0;
        loop {
            smudges += grid[left].bitxor(grid[right]).count_ones();
            if left == 0 || right == grid.len() - 1 {
                break;
            }
            left -= 1;
            right += 1;
        }
        if smudges == 1 { reflections.push(i) }
    }
    reflections
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let mut grids: Vec<Vec<Vec<u8>>> = Vec::new();
    for grid_str in input.split("\n\n") {
        grids.push(grid_str.lines().map(|line| line.as_bytes().to_vec()).collect());
    }

    let mut part1_total = 0;
    let mut part2_total = 0;

    for grid in grids {
        let grid_u64: Vec<u64> = grid.iter().map(|row| row_to_u64(row)).collect();
        let rotated = rotate_grid(&grid);
        let rotated_u64: Vec<u64> = rotated.iter().map(|row| row_to_u64(row)).collect();
        let reflections = find_reflections(&grid_u64);
        let rotated_reflections = find_reflections(&rotated_u64);
        debug!(?reflections, ?rotated_reflections);
        part1_total += rotated_reflections.iter().sum::<usize>();
        part1_total += reflections.iter().sum::<usize>() * 100;

        let part2_reflections = find_reflections_pt2(&grid_u64);
        let part2_rotated_reflections = find_reflections_pt2(&rotated_u64);
        part2_total += part2_rotated_reflections.iter().sum::<usize>();
        part2_total += part2_reflections.iter().sum::<usize>() * 100;
    }

    info!(day=13, part=1, answer=part1_total);
    info!(day=13, part=2, answer=part2_total);

    Ok(())
}
