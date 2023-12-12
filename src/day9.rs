use std::borrow::Cow;
use color_eyre::Report;
use nom::bytes::complete::take_while1;
use nom::character::complete::{newline, space1};
use nom::combinator::{all_consuming, map_res};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::terminated;
use tracing::{debug, info};

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    many1(
        terminated(
            separated_list1(
                space1,
                map_res(take_while1(|c: char| c.is_ascii_digit() || c == '-'), |s: &str| s.parse())
            ),
            newline
        )
    )(input)
}

fn difference_of_steps(steps: &[i64]) -> Vec<i64> {
    steps.windows(2).map(|w| w[1] - w[0]).collect()
}

fn predict(mut readings: Cow<[i64]>) -> (i64, i64) {
    let mut sum_of_last = *readings.last().unwrap();
    let mut firsts = vec![*readings.first().unwrap()];
    loop {
        readings = Cow::Owned(difference_of_steps(&readings));
        debug!(?readings);
        sum_of_last += readings.last().unwrap();
        firsts.push(*readings.first().unwrap());
        if readings.iter().all(|d| *d == 0) {
            break
        }
    }

    let mut first = 0;
    debug!(?firsts);
    for i in (0..firsts.len()).rev() {
        // debug!("first = {} - {} ({})", firsts[i], first, firsts[i] - first);
        first = firsts[i] - first;
    }
    (sum_of_last, first)
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let (_, histories) = all_consuming(parse_input)(&input).unwrap();
    debug!(?histories);
    let mut part1_sum = 0;
    let mut part2_sum = 0;
    for history in histories {
        debug!(?history);
        let (part1_prediction, part2_prediction) = predict(Cow::from(history));
        part1_sum += part1_prediction;
        part2_sum += part2_prediction;
        debug!(part1_prediction, part2_prediction);
    }
    info!(day=9, part=1, answer=part1_sum);

    info!(day=9, part=2, answer=part2_sum);

    Ok(())
}
