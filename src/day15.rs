use color_eyre::Report;
use nom::character::complete::{alpha1, digit1, one_of};
use nom::combinator::{all_consuming, map_res, opt};
use nom::IResult;
use nom::sequence::tuple;
use tracing::{debug, info};

fn hash(s: &str) -> u8 {
    let mut value = 0;
    for ch in s.chars() {
        value += (ch as u8) as i32;
        value *= 17;
        value %= 256;
    }
    value as u8
}

enum Operation<'a> {
    Insert(&'a str, i32),
    Remove(&'a str)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    tuple((
        alpha1,
        one_of("-="),
        opt(
            map_res(digit1, |s: &str| s.parse())
        )
    ))(input).map(|(rest, (tag, op, num))|
        (rest, match op {
                '=' => Operation::Insert(tag, num.unwrap()),
                '-' => Operation::Remove(tag),
                _ => unreachable!()
            }
        )
    )
}

pub(crate) fn solve(input: String) -> Result<(), Report> {

    let strings: Vec<&str> = input.trim().split(',').collect();

    debug!(hash_hash=hash("HASH"));

    let mut hash_total: i32 = 0;
    for s in &strings {
        hash_total += hash(s) as i32;
    }

    info!(day=1, part=1, answer=hash_total);

    let mut table: Vec<Vec<(String, i32)>> = vec![vec![]; 256];

    'op: for s in &strings {
        let (_, op) = all_consuming(parse_operation)(s).unwrap();
        match op {
            Operation::Insert(tag, value) => {
                let bucket = hash(tag) as usize;
                for (entry_tag, entry_value) in &mut table[bucket] {
                    if entry_tag == tag {
                        *entry_value = value;
                        continue 'op;
                    }
                }
                table[bucket].push((tag.to_string(), value))
            }
            Operation::Remove(tag) => {
                let bucket = hash(tag) as usize;
                table[bucket].retain(|(entry_tag, _)| entry_tag != tag);
            }
        }
    }

    let mut total = 0;
    for (idx, bucket) in table.iter().enumerate() {
        for (entry_idx, (tag, value)) in bucket.iter().enumerate() {
            debug!(tag=tag, bx=(idx + 1), slot=(entry_idx + 1), value=value);
            total += (idx + 1) * (entry_idx + 1) * (*value as usize);
        }
    }

    info!(day=1, part=2, answer=total);

    Ok(())
}
