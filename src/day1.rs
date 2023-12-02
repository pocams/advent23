use color_eyre::Report;
use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::one_of;
use nom::combinator::{all_consuming, map, value};
use nom::multi::many1;
use tracing::{debug, info};

fn decimal_digit(input: &str) -> IResult<&str, u8> {
    map(one_of("0123456789"), |ch: char| ch as u8 - 0x30)(input)
}

fn text_digit(input: &str) -> IResult<&str, u8> {
    alt((
        value(1, tag("one")),
        value(2, tag("two")),
        value(3, tag("three")),
        value(4, tag("four")),
        value(5, tag("five")),
        value(6, tag("six")),
        value(7, tag("seven")),
        value(8, tag("eight")),
        value(9, tag("nine")),
    ))(input)
}

fn optional_decimal_digit(input: &str) -> IResult<&str, Option<u8>> {
    alt((
        map(decimal_digit, Some),
        value(None, take(1usize)),
    ))(input)
}

fn optional_digit(input: &str) -> IResult<&str, Option<u8>> {
    alt((
        map(decimal_digit, Some),
        map(text_digit, Some),
        value(None, take(1usize)),
    ))(input)
}

fn calibration_value_part1(input: &str) -> Vec<Option<u8>> {
    let (_, res) = all_consuming(many1(optional_decimal_digit))(input).unwrap();
    res
}

fn calibration_value_part2(input: &str) -> Vec<Option<u8>> {
    let (_, res) = all_consuming(many1(optional_digit))(input).unwrap();
    res
}

// fn first_last_digit(s: &str) -> i64 {
//     let mut first_digit = None;
//     let mut last_digit = None;
//     for c in s.chars() {
//         if c.is_ascii_digit() {
//             let digit_value = c as u8 - 0x30;
//             if first_digit.is_none() {
//                 first_digit = Some(digit_value);
//             }
//             last_digit = Some(digit_value);
//         }
//     }
//     (first_digit.unwrap() * 10 + last_digit.unwrap()) as i64
// }

fn first_last_digit(digits: &[Option<u8>]) -> i64 {
    let mut first_digit = None;
    let mut last_digit = None;
    for digit in digits {
        if let Some(d) = digit {
            if first_digit.is_none() {
                first_digit = Some(d);
            }
            last_digit = Some(d);
        }
    }
    // Allow for no digits on the line - otherwise we fail parsing part 2 input with part 1 rules
    match (first_digit, last_digit) {
        (Some(f), Some(l)) => {
            println!("{}{}", f, l);
            debug!(first=?f, last=?l);
            (f * 10 + l) as i64
        },
        _ => 0,
    }
}

pub(crate) fn solve(input: String) -> Result<(), Report> {

    let part1_sum: i64 = input.lines().map(calibration_value_part1).map(|digits| first_last_digit(&digits)).sum();
    
    info!(day=1, part=1, answer=part1_sum);

    let part2_sum: i64 = input.lines()
        .map(|line| {
            let parsed = calibration_value_part2(line);
            debug!(%line, ?parsed);
            parsed
        })
        .map(|digits| {
            first_last_digit(&digits)
        }).sum();
    
    info!(day=1, part=2, answer=part2_sum);

    Ok(())
}
