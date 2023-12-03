use color_eyre::Report;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::combinator::{map, rest, value};
use nom::IResult;
use nom::sequence::tuple;
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

fn starts_with_decimal_digit(input: &str) -> IResult<&str, u8> {
    tuple((decimal_digit, rest))(input).map(|(rest, (digit, _))| (rest, digit))
}

fn starts_with_any_digit(input: &str) -> IResult<&str, u8> {
    tuple((alt((decimal_digit, text_digit)), rest))(input).map(|(rest, (digit, _))| (rest, digit))
}

fn first(input: &str, parser: impl Fn(&str) -> IResult<&str, u8>) -> Option<u8> {
    for i in 0..=input.len() {
        if let Ok((_, digit)) = parser(&input[i..input.len()]) {
            return Some(digit)
        }
    }
    debug!("no first digit for {:?}", input);
    None
}

fn last(input: &str, parser: impl Fn(&str) -> IResult<&str, u8>) -> Option<u8> {
    for i in 0..=input.len() {
        if let Ok((_, digit)) = parser(&input[input.len() - i..input.len()]) {
            return Some(digit)
        }
    }
    debug!("no last digit for {:?}", input);
    None
}

fn part1_value(input: &str) -> u8 {
    first(input, starts_with_decimal_digit).unwrap_or(0) * 10 + last(input, starts_with_decimal_digit).unwrap_or(0)
}

fn part2_value(input: &str) -> u8 {
    first(input, starts_with_any_digit).unwrap() * 10 + last(input, starts_with_any_digit).unwrap()
}

pub(crate) fn solve(input: String) -> Result<(), Report> {

    let part1_sum: i64 = input.lines().map(|line| part1_value(line) as i64).sum();
    
    info!(day=1, part=1, answer=part1_sum);

    let part2_sum: i64 = input.lines().map(|line| part2_value(line) as i64).sum();

    info!(day=1, part=2, answer=part2_sum);

    Ok(())
}
