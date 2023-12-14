use std::cell::RefCell;
use std::rc::Rc;
use color_eyre::Report;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline, one_of, space1};
use nom::combinator::{all_consuming, map_res};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::{terminated, tuple};
use tracing::{debug, info};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Spring {
    Ok,
    Damaged,
    Unknown
}

impl TryFrom<char> for Spring {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Spring::Ok),
            '#' => Ok(Spring::Damaged),
            '?' => Ok(Spring::Unknown),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone)]
struct Row {
    springs: Vec<Spring>,
    groups: Vec<i32>
}

fn parse_row(input: &str) -> IResult<&str, Row> {
    tuple((
        many1(
            map_res(one_of(".#?"), Spring::try_from)
        ),
        space1,
        separated_list1(
            tag(","),
            map_res(digit1, |n: &str| n.parse())
        )
    ))(input).map(
        |(rest, (springs, _, groups))|
        (rest, Row { springs, groups })
    )
}

// fn calculate_groups(springs: &[Spring]) -> Vec<i32> {
//     let mut groups = Vec::new();
//     let mut current_group = 0;
//     for spring in springs {
//         match spring {
//             Spring::Ok => {
//                 if current_group > 0 {
//                     groups.push(current_group);
//                     current_group = 0;
//                 }
//             }
//             Spring::Damaged => {
//                 current_group += 1;
//             }
//             Spring::Unknown => panic!("Can't calculate groups with unknown springs")
//         }
//     }
//     if current_group > 0 {
//         groups.push(current_group);
//     }
//     groups
// }

#[allow(dead_code)]
fn show(springs: &[Spring], groups: &[i32], comment: &str) {
    for spring in springs {
        match spring {
            Spring::Ok => print!("."),
            Spring::Damaged => print!("#"),
            Spring::Unknown => print!("?"),
        }
    }
    if groups.is_empty() {
        print!(" -");
    } else {
        print!(" {}", groups[0]);
        for g in &groups[1..] {
            print!(",{}", g);
        }
    }
    if !comment.is_empty() {
        print!("\t\t{}", comment);
    }
    println!();
}

impl Row {
    // fn count_possibilities(&self) -> i32 {
    //     let total_damaged: usize = self.groups.iter().map(|g| *g as usize).sum();
    //     let total_known_damaged = self.springs.iter().filter(|s| matches!(s, Spring::Damaged)).count();
    //     let unknowns: Vec<_> = self.springs.iter()
    //         .enumerate()
    //         .filter_map(|(i, spring)| if matches!(spring, Spring::Unknown) { Some(i) } else { None })
    //         .collect();
    //     let mut possibilities = 0;
    //     show(&self.springs);
    //     debug!(?unknowns, groups=?self.groups, k=(total_damaged - total_known_damaged));
    //     for perm in unknowns.iter().combinations(total_damaged - total_known_damaged) {
    //         // debug!(?perm);
    //         let mut potential_springs = self.springs.clone();
    //         for &pos in perm {
    //             potential_springs[pos] = Spring::Damaged;
    //         }
    //         for remaining in potential_springs.iter_mut() {
    //             if *remaining == Spring::Unknown {
    //                 *remaining = Spring::Ok
    //             }
    //         }
    //         // show(&potential_springs);
    //         // debug!(groups=?calculate_groups(&potential_springs));
    //         if calculate_groups(&potential_springs) == self.groups {
    //             possibilities += 1;
    //         }
    //     }
    //     possibilities
    // }

    fn count_possibilities(&self) -> i64 {
        let cache = vec![vec![-1; self.groups.len() + 1]; self.springs.len() + 1];
        count_matches(&self.springs, &self.groups, None, Rc::new(RefCell::new(cache)))
    }

    fn unfold(&self) -> Row {
        let mut springs = Vec::new();
        springs.extend_from_slice(&self.springs);
        for _ in 0..4 {
            springs.push(Spring::Unknown);
            springs.extend_from_slice(&self.springs);
        }
        let mut groups = Vec::new();
        for _ in 0..5 { groups.extend_from_slice(&self.groups); }
        Row { springs, groups }
    }
}

fn count_matches(springs: &[Spring], groups: &[i32], match_first_as: Option<Spring>, cache: Rc<RefCell<Vec<Vec<i64>>>>) -> i64 {
    // show(springs, groups, &format!("{:?}", match_first_as));
    if springs.is_empty() {
        return if groups.is_empty() { 1 } else { 0 }
    }

    if match_first_as.is_none() {
        let cached = cache.borrow()[springs.len()][groups.len()];
        if cached >= 0 { return cached }
    }

    let count = match match_first_as.unwrap_or(springs[0]) {
        Spring::Ok => count_matches(&springs[1..], groups, None, cache.clone()),
        Spring::Damaged => {
            if groups.is_empty() { return 0 }
            let grouplen = groups[0] as usize;
            if springs.len() < grouplen { return 0 }
            if springs.iter().take(grouplen).any(|s| *s == Spring::Ok) {
                // No matches to be found here, group is too small
                return 0
            }
            // There are <grouplen> damaged or unknown springs at the beginning; now make sure there's a
            // possible gap at the end of the group
            if springs.len() > grouplen && springs[grouplen] == Spring::Damaged {
                // No gap at the end of the group
                return 0
            }
            // Okay, we matched this group, continue matching from here on
            // debug!(?springs, ?groups);
            if springs.len() > grouplen {
                // Skip one extra spring to account for the gap
                count_matches(&springs[grouplen+1..], &groups[1..], None, cache.clone())
            } else {
                // We're at the end of the springs array, &springs[grouplen..] will be empty
                count_matches(&springs[grouplen..], &groups[1..], None, cache.clone())
            }
        }
        Spring::Unknown => {
            count_matches(springs, groups, Some(Spring::Ok), cache.clone()) + count_matches(springs, groups, Some(Spring::Damaged), cache.clone())
        }
    };

    if match_first_as.is_none() {
        cache.borrow_mut()[springs.len()][groups.len()] = count;
    }
    count
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let (_, rows) = all_consuming(
        many1(
            terminated(
                parse_row,
                newline
            )
        )
    )(&input).unwrap();

    debug!(?rows);

    let mut total_possibilities = 0;
    for row in &rows {
        total_possibilities += row.count_possibilities();
    }

    info!(day=12, part=1, answer=total_possibilities);

    let part2_rows: Vec<_> = rows.iter().map(|r| r.unfold()).collect();
    let mut part2_possibilities = 0;

    for row in &part2_rows {
        // show(&row.springs, &row.groups, "");
        let possibilities = row.count_possibilities();
        debug!(possibilities);
        part2_possibilities += possibilities;
    }
    info!(day=12, part=2, answer=part2_possibilities);

    Ok(())
}
