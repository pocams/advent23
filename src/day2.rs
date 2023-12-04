use std::ops::Add;

use color_eyre::Report;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0, space1};
use nom::combinator::{map_res, value};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use tracing::{debug, info};

#[derive(Debug, Copy, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Eq, PartialEq, Default)]
struct CubeSet {
    red: i32,
    green: i32,
    blue: i32,
}

impl Add for CubeSet {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        CubeSet {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl CubeSet {
    fn contains(&self, other: &CubeSet) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    fn add_cubes(&mut self, count: i32, color: Color) {
        match color {
            Color::Red => self.red += count,
            Color::Green => self.blue += count,
            Color::Blue => self.green += count,
        }
    }

    fn ensure_contains(&mut self, other: &CubeSet) {
        if self.red < other.red { self.red = other.red }
        if self.green < other.green { self.green = other.green }
        if self.blue < other.blue { self.blue = other.blue }
    }

    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

fn parse_color(input: &str) -> IResult<&str, (i32, Color)> {
    tuple((
        map_res(digit1, |s: &str| s.parse()),
        space1,
        alt((
            value(Color::Red, tag("red")),
            value(Color::Green, tag("green")),
            value(Color::Blue, tag("blue")),
        ))
    ))(input).map(|(rest, (count, _, color))| (rest, (count, color)))
}

fn parse_cubeset(input: &str) -> IResult<&str, CubeSet> {
    separated_list1(
        tuple((
            tag(","),
            space0
        )),
        parse_color
    )(input).map(|(rest, colors)| {
        let mut cs = CubeSet::default();
        for (count, color) in colors {
            cs.add_cubes(count, color);
        }
        (rest, cs)
    })
}

fn parse_game(input: &str) -> IResult<&str, (i32, Vec<CubeSet>)> {
    tuple((
        tag("Game "),
        map_res(digit1, |s: &str| s.parse()),
        tag(": "),
        separated_list1(
            tuple((tag(";"), space0)),
            parse_cubeset
        )
    ))(input).map(|(rest, (_, n, _, cubesets))|
        (rest, (n, cubesets))
    )
}

pub(crate) fn solve(input: String) -> Result<(), Report> {

    let (_, bag) = parse_cubeset("12 red, 13 green, 14 blue").unwrap();
    debug!(bag=?bag);

    let mut part1 = 0;
    'games: for line in input.lines() {
        let (_, (game_id, cubesets)) = parse_game(line).expect("unparseable game");
        for cubeset in cubesets {
            if !bag.contains(&cubeset) {
                debug!(bag=?bag, cubeset=?cubeset, "impossible");
                continue 'games;
            }
        }
        part1 += game_id;
    }

    info!(day=2, part=1, answer=part1);

    let mut part2 = 0;
    for line in input.lines() {
        let mut minimum_set = CubeSet::default();
        let (_, (_, cubesets)) = parse_game(line).expect("unparseable game");
        for cubeset in cubesets {
            minimum_set.ensure_contains(&cubeset);
        }
        part2 += minimum_set.power();
    }

    info!(day=2, part=2, answer=part2);

    Ok(())
}
