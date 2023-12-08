use color_eyre::Report;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline, space1};
use nom::combinator::{all_consuming, map_res};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::{terminated, tuple};
use tracing::{debug, info};

#[derive(Debug)]
struct Card {
    #[allow(dead_code)]
    id: i32,
    winning_numbers: Vec<i32>,
    have_numbers: Vec<i32>,
    copies: i32,
}

impl Card {
    fn winning_number_count(&self) -> i32 {
        let mut matches = 0;
        for have_number in &self.have_numbers {
            for winning_number in &self.winning_numbers {
                if have_number == winning_number {
                    matches += 1;
                }
            }
        }
        matches
    }

    fn part1_value(&self) -> i32 {
        let matches = self.winning_number_count();
        if matches > 0 {
            1 << (matches - 1)
        } else {
            0
        }
    }
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    tuple((
        tag("Card"),
        space1,
        map_res(digit1, |s: &str| s.parse()),
        tag(":"),
        space1,
        separated_list1(
            space1,
            map_res(digit1, |s: &str| s.parse()),
        ),
        space1,
        tag("|"),
        space1,
        separated_list1(
            space1,
            map_res(digit1, |s: &str| s.parse()),
        ),
    ))(input).map(|(rest, (_, _, id, _, _, winning_numbers, _, _, _, have_numbers))|
        (rest, (Card { id, winning_numbers, have_numbers, copies: 1 }))
    )
}

pub(crate) fn solve(input: String) -> Result<(), Report> {

    let (_, mut cards) = all_consuming(
        many1(
            terminated(parse_card, newline)
        )
    )(&input).unwrap();

    debug!(?cards);

    let total: i32 = cards.iter().map(|c| c.part1_value()).sum();

    info!(day=4, part=1, answer=total);

    for i in 0..cards.len() {
        for n in 0..cards[i].winning_number_count() as usize {
            cards[i + n + 1].copies += cards[i].copies;
        }
    }

    let total_cards: i32 = cards.iter().map(|c| c.copies).sum();
    info!(day=4, part=2, answer=total_cards);

    Ok(())
}
