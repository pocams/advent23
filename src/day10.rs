use color_eyre::owo_colors::OwoColorize;
use color_eyre::Report;
use tracing::{info, warn};

const NORTH_SOUTH: u8 = b'|';
const EAST_WEST: u8 = b'-';
const NORTH_EAST: u8 = b'L';
const NORTH_WEST: u8 = b'J';
const SOUTH_EAST: u8 = b'F';
const SOUTH_WEST: u8 = b'7';
const GROUND: u8 = b'.';
const START: u8 = b'S';

fn find_coords<T>(map: &[Vec<T>], pred: impl Fn(&T) -> bool) -> Option<(usize, usize)> {
    for (row_num, row) in map.iter().enumerate() {
        for (col_num, val) in row.iter().enumerate() {
            if pred(val) {
                return Some((row_num, col_num))
            }
        }
    }
    None
}

fn start_to_pipe(map: &mut Vec<Vec<u8>>) {
    let (row, col) = find_coords(map, |&c| c == START).unwrap();
    let has_north = row > 0 && [NORTH_SOUTH, SOUTH_EAST, SOUTH_WEST].contains(&map[row-1][col]);
    let has_south = row + 1 < map.len() && [NORTH_SOUTH, NORTH_EAST, NORTH_WEST].contains(&map[row+1][col]);
    let has_west = col > 0 && [EAST_WEST, NORTH_EAST, SOUTH_EAST].contains(&map[row][col-1]);
    let has_east = col + 1 < map[0].len() && [EAST_WEST, NORTH_WEST, SOUTH_WEST].contains(&map[row][col+1]);
    map[row][col] = match (has_north, has_south, has_west, has_east) {
        (true, true, false, false) => NORTH_SOUTH,
        (true, false, true, false) => NORTH_WEST,
        (true, false, false, true) => NORTH_EAST,
        (false, true, true, false) => SOUTH_WEST,
        (false, true, false, true) => SOUTH_EAST,
        (false, false, true, true) => EAST_WEST,
        _ => panic!("can't convert {:?} to pipe", (has_north, has_south, has_east, has_west))
    };
}

fn offsets(map: &Vec<Vec<u8>>, (row, col): (usize, usize)) -> Vec<(usize, usize)> {
    let mut offsets = Vec::new();
    let ch_type = map[row][col];
    if (ch_type == NORTH_SOUTH || ch_type == NORTH_WEST || ch_type == NORTH_EAST) && row > 0 { offsets.push((row - 1, col)) }
    if (ch_type == NORTH_SOUTH || ch_type == SOUTH_WEST || ch_type == SOUTH_EAST) && row + 1 < map.len() { offsets.push((row + 1, col)) }
    if (ch_type == EAST_WEST || ch_type == NORTH_WEST || ch_type == SOUTH_WEST) && col > 0 { offsets.push((row, col - 1)) }
    if (ch_type == EAST_WEST || ch_type == NORTH_EAST || ch_type == SOUTH_EAST) && col + 1 < map[0].len() { offsets.push((row, col + 1)) }

    if offsets.len() == 1 || offsets.len() == 3 {
        warn!(char=?ch_type as char, row, col, "weird offsets len");
    }
    offsets
}

fn show(map: &[Vec<u8>], steps: &[Vec<Option<i32>>]) {
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

fn count_inner(clean_map: &[Vec<u8>]) -> i32 {
    let mut count = 0;
    let mut east_corner = None;
    for row in clean_map.iter() {
        let mut inside = false;
        for &map_cell in row.iter() {
            match map_cell {
                GROUND => if inside { count += 1 },
                NORTH_SOUTH => inside = !inside,
                EAST_WEST => {},
                NORTH_EAST => east_corner = Some(NORTH_EAST),
                SOUTH_EAST => east_corner = Some(SOUTH_EAST),
                NORTH_WEST => {
                    if east_corner.unwrap() == SOUTH_EAST {
                        inside = !inside
                    }
                    east_corner = None;
                },
                SOUTH_WEST => {
                    if east_corner.unwrap() == NORTH_EAST {
                        inside = !inside
                    }
                    east_corner = None;
                }
                _ => panic!("unexpected map cell {:?}", map_cell as char),
            }
            if inside {
                print!("{}", (map_cell as char).white().on_blue())
            } else {
                print!("{}", (map_cell as char).white().on_black())
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
    start_to_pipe(&mut map);
    let mut to_check = vec![start];
    steps[start.0][start.1] = Some(0);

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

    let clean_map: Vec<Vec<u8>> = map.iter().zip(steps.iter()).map(
        |(row, steps_row)| row.iter().zip(steps_row.iter()).map(
            |(ch, step)| if step.is_some() { *ch } else { GROUND }
        ).collect()
    ).collect();

    let inner = count_inner(&clean_map);
    info!(day=1, part=2, answer=inner);

    Ok(())
}
