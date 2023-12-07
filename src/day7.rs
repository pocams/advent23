use std::cmp::Ordering;
use color_eyre::Report;
use nom::character::complete::{digit1, newline, one_of, space1};
use nom::combinator::{all_consuming, map_res};
use nom::IResult;
use nom::multi::{count, many1};
use nom::sequence::{terminated, tuple};
use tracing::{debug, info};

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<i8>,
    bid: i64
}

const JOKER_VALUE: i8 = 1;

impl Hand {
    fn value_before_jokers(&self) -> Value {
        let mut cards = self.cards.clone();
        cards.sort();
        let mut counts = vec![1];
        for i in 1..cards.len() {
            if cards[i] == cards[i-1] {
                *counts.last_mut().unwrap() += 1
            } else {
                counts.push(1);
            }
        }

        counts.sort();
        match counts.as_slice() {
            [5] => Value::FiveOfAKind,
            [1, 4] => Value::FourOfAKind,
            [2, 3] => Value::FullHouse,
            [1, 1, 3] => Value::ThreeOfAKind,
            [1, 2, 2] => Value::TwoPair,
            [1, 1, 1, 2] => Value::OnePair,
            [1, 1, 1, 1, 1] => Value::Nothing,
            _ => panic!("unexpected counts {counts:?}")
        }
    }

    fn value(&self) -> Value {
        let jokers = self.cards.iter().filter(|c| **c == JOKER_VALUE).count();
        match (self.value_before_jokers(), jokers) {
            // No jokers means the value doesn't change
            (v, 0) => v,
            // Five jokers is still five of a kind
            (Value::FiveOfAKind, 5) => Value::FiveOfAKind,
            // 4 of a kind + 1 joker becomes five of a kind
            (Value::FourOfAKind, 1) => Value::FiveOfAKind,
            // 4 jokers + 1 (anything) becomes five of a kind
            (Value::FourOfAKind, 4) => Value::FiveOfAKind,
            // Either 3 of a kind and 2 jokers, or one pair and 3 jokers becomes 5 of a kind
            (Value::FullHouse, 2) => Value::FiveOfAKind,
            (Value::FullHouse, 3) => Value::FiveOfAKind,
            // 3 jokers plus 2 other cards becomes 4 of a kind
            (Value::ThreeOfAKind, 3) => Value::FourOfAKind,
            // 3 of a kind with 1 joker becomes 4 of a kind
            (Value::ThreeOfAKind, 1) => Value::FourOfAKind,
            // Two pair where 1 pair is jokers becomes 4 of a kind
            (Value::TwoPair, 2) => Value::FourOfAKind,
            // Two pair plus 1 joker becomes a full house
            (Value::TwoPair, 1) => Value::FullHouse,
            // One pair of jokers plus 3 other cards becomes 3 of a kind
            (Value::OnePair, 2) => Value::ThreeOfAKind,
            // One pair plus 1 joker becomes 3 of a kind
            (Value::OnePair, 1) => Value::ThreeOfAKind,
            // Nothing with 1 joker becomes 1 pair
            (Value::Nothing, 1) => Value::OnePair,
            (v, j) => panic!("Unexpected: {:?} with {} jokers", v, j)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.value().cmp(&other.value()) {
            Ordering::Equal => {
                self.cards.cmp(&other.cards)
            }
            less_or_greater => less_or_greater
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Value {
    Nothing,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn parse_card(input: &str) -> IResult<&str, i8> {
    one_of("23456789TJQKA")(input)
        .map(|(rest, ch)| (rest, card_to_value(ch)))
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    tuple((
        count(parse_card, 5),
        space1,
        map_res(digit1, |s: &str| s.parse())
    ))(input).map(|(rest, (cards, _, bid))|
        (rest, Hand { cards, bid } )
    )
}

fn card_to_value(card: char) -> i8 {
    match card {
        '2' ..= '9' => ((card as u8) - 0x30) as i8,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("unexpected card {card}")
    }
}

pub(crate) fn solve(input: String) -> Result<(), Report> {

    let mut hands = all_consuming(
        many1(
            terminated(
                parse_hand,
                newline
            ))
    )(&input).map(|(_, hands)| hands).unwrap();

    hands.sort();

    let mut total_score = 0;
    for (index, hand) in hands.iter().enumerate() {
        let score = hand.bid * (index + 1) as i64;
        debug!(hand=?hand, value=?hand.value(), score=?score);
        total_score += score;
    }

    info!(day=1, part=1, answer=total_score);

    for hand in &mut hands {
        for c in hand.cards.iter_mut() {
            if *c == 11 {
                *c = JOKER_VALUE
            }
        }
    }

    hands.sort();
    let mut total_score_part2 = 0;
    for (index, hand) in hands.iter().enumerate() {
        let score = hand.bid * (index + 1) as i64;
        debug!(hand=?hand, value=?hand.value(), score=?score);
        total_score_part2 += score;
    }

    info!(day=1, part=2, answer=total_score_part2);

    Ok(())
}
