use color_eyre::Report;
use tracing::{debug, info};

const GALAXY: u8 = b'#';
const SPACE: u8 = b'.';

const PART2_FACTOR: usize = 1000000;

pub(crate) fn solve(input: String) -> Result<(), Report> {

    let mut map = Vec::new();
    for line in input.lines() {
        map.push(line.as_bytes().to_vec());
    }

    let mut galaxies = Vec::new();
    let mut doubled_rows = Vec::new();
    let mut doubled_cols = Vec::new();

    for (row_num, row) in map.iter().enumerate() {
        for (col_num, &ch) in row.iter().enumerate() {
            if ch == GALAXY {
                galaxies.push((row_num, col_num));
            }
        }

        if row.iter().all(|&c| c == SPACE) {
            doubled_rows.push(row_num);
        }
    }

    for col_num in 0..map[0].len() {
        if (0..map.len()).all(|row_num| map[row_num][col_num] == SPACE) {
            doubled_cols.push(col_num);
        }
    }

    debug!(?doubled_rows, ?doubled_cols);

    let mut total_distance = 0;

    for (row, col) in &galaxies {
        for (other_row, other_col) in &galaxies {
            if other_row > row || (other_row == row && other_col > col) {
                let mut distance = row.abs_diff(*other_row) + col.abs_diff(*other_col);
                distance += doubled_rows.iter().filter(|dr| row.min(other_row) < *dr && *dr < row.max(other_row)).count();
                distance += doubled_cols.iter().filter(|dc| col.min(other_col) < *dc && *dc < col.max(other_col)).count();
                total_distance += distance;
            }
        }
    }

    info!(day=11, part=1, answer=total_distance);

    total_distance = 0;

    for (row, col) in &galaxies {
        for (other_row, other_col) in &galaxies {
            if other_row > row || (other_row == row && other_col > col) {
                let mut distance = row.abs_diff(*other_row) + col.abs_diff(*other_col);
                distance += doubled_rows.iter().filter(|dr| row.min(other_row) < *dr && *dr < row.max(other_row)).count() * (PART2_FACTOR - 1);
                distance += doubled_cols.iter().filter(|dc| col.min(other_col) < *dc && *dc < col.max(other_col)).count() * (PART2_FACTOR - 1);
                total_distance += distance;
            }
        }
    }

    info!(day=11, part=2, answer=total_distance);

    Ok(())
}
