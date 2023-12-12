use color_eyre::owo_colors::OwoColorize;
use color_eyre::Report;
use tracing::{debug, info, warn};

const NORTH_SOUTH: u8 = b'|';
const EAST_WEST: u8 = b'-';
const NORTH_EAST: u8 = b'L';
const NORTH_WEST: u8 = b'J';
const SOUTH_EAST: u8 = b'F';
const SOUTH_WEST: u8 = b'7';
const GROUND: u8 = b'.';
const START: u8 = b'S';

fn find_coords<T>(map: &Vec<Vec<T>>, pred: impl Fn(&T) -> bool) -> Option<(usize, usize)> {
    for (row_num, row) in map.iter().enumerate() {
        for (col_num, val) in row.iter().enumerate() {
            if pred(val) {
                return Some((row_num, col_num))
            }
        }
    }
    None
}

fn start_offsets(map: &Vec<Vec<u8>>, (row, col): (usize, usize)) -> Vec<(usize, usize)> {
    let mut offsets = Vec::new();
    if row > 0 {
        if map[row-1][col] == NORTH_SOUTH || map[row-1][col] == SOUTH_WEST || map[row-1][col] == SOUTH_EAST {
            debug!(north=?map[row-1][col] as char);
            offsets.push((row - 1, col))
        }
    }
    if row + 1 < map.len() {
        if map[row+1][col] == NORTH_SOUTH || map[row+1][col] == NORTH_WEST || map[row+1][col] == NORTH_EAST {
            debug!(south=?map[row+1][col] as char);
            offsets.push((row + 1, col))
        }
    }
    if col > 0 {
        if map[row][col-1] == EAST_WEST || map[row][col-1] == NORTH_EAST || map[row][col-1] == SOUTH_EAST {
            debug!(east=?map[row][col-1] as char);
            offsets.push((row, col - 1))
        }
    }
    if col + 1 < map[0].len() {
        if map[row][col+1] == EAST_WEST || map[row][col+1] == NORTH_WEST || map[row][col+1] == SOUTH_WEST {
            debug!(west=?map[row][col+1] as char);
            offsets.push((row, col + 1))
        }
    }
    assert_eq!(offsets.len(), 2);
    offsets
}

fn offsets(map: &Vec<Vec<u8>>, (row, col): (usize, usize)) -> Vec<(usize, usize)> {
    let mut offsets = Vec::new();
    let ch_type = map[row][col];
    if ch_type == NORTH_SOUTH || ch_type == NORTH_WEST || ch_type == NORTH_EAST {
        if row > 0 { offsets.push((row - 1, col)) }
    }
    if ch_type == NORTH_SOUTH || ch_type == SOUTH_WEST || ch_type == SOUTH_EAST {
        if row + 1 < map.len() { offsets.push((row + 1, col)) }
    }
    if ch_type == EAST_WEST || ch_type == NORTH_WEST || ch_type == SOUTH_WEST {
        if col > 0 { offsets.push((row, col - 1)) }
    }
    if ch_type == EAST_WEST || ch_type == NORTH_EAST || ch_type == SOUTH_EAST {
        if col + 1 < map[0].len() { offsets.push((row, col + 1)) }
    }

    if offsets.len() == 1 || offsets.len() == 3 {
        warn!(char=?ch_type as char, row, col, "weird offsets len");
    }
    offsets
}

fn show(map: &Vec<Vec<u8>>, steps: &Vec<Vec<Option<i32>>>) {
    for (map_row, steps_row) in map.iter().zip(steps.iter()) {
        for (map_cell, steps_cell) in map_row.iter().zip(steps_row.iter()) {
            match steps_cell {
                Some(s) => print!("{}", s.bright_yellow().on_blue()),
                None => print!("{}", (*map_cell as char).bright_black())
            }
        }
        println!();
    }
}

fn count_inner(map: &Vec<Vec<u8>>, steps: &Vec<Vec<Option<i32>>>) -> i32 {
    let mut count = 0;
    for (map_row, steps_row) in map.iter().zip(steps.iter()) {
        let mut inside = false;
        for (&map_cell, steps_cell) in map_row.iter().zip(steps_row.iter()) {
            if steps_cell.is_some() {
                print!("{}", (map_cell as char).bright_yellow().on_red());
                inside = !inside
            } else {
                if inside {
                    print!("{}", (map_cell as char).bright_white().on_blue());
                    count += 1;
                } else {
                    print!("{}", (map_cell as char).white().on_black());
                }
            }
        }
        println!();
    }
    count
}

pub(crate) fn solve(input: String) -> Result<(), Report> {

    let mut map = Vec::new();
    for line in input.lines() {
        map.push(line.as_bytes().to_vec());
    }

    let mut steps: Vec<Vec<Option<i32>>> = vec![
        vec![None; map[0].len()];
        map.len()
    ];

    show(&map, &steps);

    let start = find_coords(&map, |&c| c == START).unwrap();
    steps[start.0][start.1] = Some(0);
    let mut to_check = start_offsets(&map, start);
    for &(row, col) in &to_check {
        steps[row][col] = Some(1);
    }

    while let Some((row, col)) = to_check.pop() {
        let step = steps[row][col].unwrap();
        for offset in offsets(&map, (row, col)) {
            if steps[offset.0][offset.1].unwrap_or(i32::MAX) > step + 1 {
                steps[offset.0][offset.1] = Some(step + 1);
                to_check.push((offset.0, offset.1));
            }
        }
    }

    show(&map, &steps);

    let max_steps = steps.iter().flat_map(|s| s.iter().map(|s| s.unwrap_or(0)).max()).max().unwrap();
    info!(day=1, part=1, answer=max_steps);

    let inner = count_inner(&map, &steps);
    info!(day=1, part=2, answer=inner);

    Ok(())
}
