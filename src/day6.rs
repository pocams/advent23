use color_eyre::Report;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline, space1};
use nom::combinator::{all_consuming, map_res};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use tracing::{debug, info};

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64
}

impl Race {
    fn distance_traveled(&self, acceleration_time: i64) -> i64 {
        (self.time - acceleration_time) * acceleration_time
    }

    fn win_states(&self) -> i64 {
        let mut states = 0;
        for acceleration_time in 0..self.time {
            if self.distance_traveled(acceleration_time) > self.distance {
                states += 1;
            }
        }
        states
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Race>> {
    tuple((
        tag("Time:"),
        space1,
        separated_list1(
            space1,
            map_res(digit1, |s: &str| s.parse())
        ),
        newline,
        tag("Distance:"),
        space1,
        separated_list1(
            space1,
            map_res(digit1, |s: &str| s.parse())
        ),
        newline,
    ))(input).map(|(rest, (_, _, times, _, _, _, distances, _))|
        (rest, times.iter().zip(distances.iter()).map(|(&time, &distance)| Race { time, distance }).collect())
    )
}

pub(crate) fn solve(input: String) -> Result<(), Report> {

    let (_, races) = all_consuming(parse_input)(&input).unwrap();
    debug!(?races);

    let mut state_product = 1;
    for race in &races {
        let states = race.win_states();
        debug!(?race, states);
        state_product *= states;
    }

    info!(day=6, part=1, answer=state_product);

    let part2_time: String = races.iter().map(|r| r.time.to_string()).collect();
    let part2_distance: String = races.iter().map(|r| r.distance.to_string()).collect();

    let part2_race = Race { time: part2_time.parse().unwrap(), distance: part2_distance.parse().unwrap() };
    debug!(?part2_race);
    let win_states = part2_race.win_states();

    info!(day=6, part=2, answer=win_states);

    Ok(())
}
