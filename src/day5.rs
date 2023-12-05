use color_eyre::Report;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{digit1, newline, space1};
use nom::combinator::{all_consuming, map_res};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::{terminated, tuple};
use tracing::{debug, info};

#[derive(Debug)]
struct Range {
    start: i64,
    length: i64,
    shift: i64,
}

impl Range {
    fn empty() -> Range {
        Range {
            start: 0,
            length: i64::MAX,
            shift: 0
        }
    }

    fn end(&self) -> i64 {
        self.start + self.length
    }

    fn map_value(&self, value: i64) -> Option<i64> {
        if value >= self.start && value < self.end() {
            Some(value + self.shift)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Day5Map {
    from: String,
    to: String,
    ranges: Vec<Range>
}

impl Day5Map {
    fn new(from: String, to: String, mut ranges: Vec<Range>) -> Day5Map {
        ranges.sort_by(|a, b| a.start.cmp(&b.start));
        let mut new_ranges = Vec::new();
        let mut last_end = 0;
        for r in ranges {
            let missing_length = r.start - last_end;
            if missing_length > 1 {
                new_ranges.push(Range { start: last_end, length: missing_length, shift: 0 })
            }
            last_end = r.end();
            new_ranges.push(r);
        }
        if last_end != i64::MAX {
            new_ranges.push(Range {
                start: last_end,
                length: i64::MAX - last_end,
                shift: 0,
            });
        }
        Day5Map { from, to, ranges: new_ranges }
    }

    fn range_for(&self, value: i64) -> &Range {
        for r in &self.ranges {
            if r.map_value(value).is_some() { return r }
        }
        debug!(?self.ranges);
        panic!("missing range for {}", value);
    }

    fn flatten(&self, other: &Day5Map) -> Day5Map {
        if self.to != other.from {
            panic!("can't flatten map to {} with map from {}", self.to, other.from)
        }

        let mut new_ranges = Vec::new();
        for range in &self.ranges {
            let mut pt = range.start;
            while pt < range.end() {
                let mapped = range.map_value(pt).unwrap();
                let other_range = other.range_for(mapped);
                debug!(?other_range);
                let shift = range.shift + other_range.shift;
                let length = (range.end() - pt).min(other_range.end() - mapped);
                new_ranges.push(Range {
                    start: pt,
                    length,
                    shift
                });
                debug!(pt, ?range, length);
                debug!(?new_ranges);
                pt = pt + length;
            }
        }

        Day5Map::new(self.from.to_string(), other.to.to_string(), new_ranges)
    }
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<i64>> {
    terminated(
        tuple((
            tag("seeds:"),
            space1,
            separated_list1(
                space1,
                map_res(digit1, |s: &str| s.parse())
            )
        )),
        newline
    )(input).map(|(rest, (_, _, seeds))|
        (rest, seeds)
    )
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    tuple((
        map_res(digit1, |s: &str| s.parse::<i64>()),
        space1,
        map_res(digit1, |s: &str| s.parse::<i64>()),
        space1,
        map_res(digit1, |s: &str| s.parse::<i64>()),
    ))(input).map(|(rest, (destination_start, _, source_start, _, length))|
        (rest, Range { start: source_start, length, shift: destination_start - source_start })
    )
}

fn parse_map(input: &str) -> IResult<&str, Day5Map> {
    terminated(
        tuple((
            take_while(|c: char| c.is_alphabetic()),
            tag("-to-"),
            take_while(|c: char| c.is_alphabetic()),
            tag(" map:"),
            newline,
            many1(
                terminated(
                    parse_range,
                    newline
                )
            )
            )),
        newline
    )(input).map(|(rest, (from, _, to, _, _, ranges))|
        (rest, Day5Map::new(from.to_string(), to.to_string(), ranges))
    )
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let (_, (seeds, maps)) = all_consuming(tuple((
        parse_seeds,
        newline,
        many1(parse_map)
    )))(&input).map(|(rest, (seeds, _, maps))|
        (rest, (seeds, maps))
    ).unwrap();

    debug!(seeds=?seeds, maps=?maps);

    let dest_map = "location";

    // let mut lowest_location = None;
    //
    // for seed in &seeds {
    //     let mut current_map = "seed";
    //     let mut value = *seed;
    //     while current_map != dest_map {
    //         for map in &maps {
    //             if map.from == current_map {
    //                 let new_value = map.range_for(value).map_value(value).unwrap();
    //                 debug!("mapping {} {} to {} {}", map.from, value, map.to, new_value);
    //                 current_map = &map.to;
    //                 value = new_value;
    //             }
    //         }
    //     }
    //     if lowest_location.is_none() || lowest_location.unwrap() > value {
    //         lowest_location = Some(value);
    //     }
    // }
    //
    // info!(day=5, part=1, answer=lowest_location.unwrap());

    let mut flat_map = Day5Map::new("seed".to_string(), "seed".to_string(), vec![Range::empty()]);
    while flat_map.to != dest_map {
        for map in &maps {
            if map.from == flat_map.to {
                flat_map = flat_map.flatten(map);
            }
        }
    }

    let mut lowest_location = None;

    for seed in &seeds {
        let value = flat_map.range_for(*seed).map_value(*seed).unwrap();
        if lowest_location.is_none() || lowest_location.unwrap() > value {
            lowest_location = Some(value);
        }
    }

    info!(day=5, part=1, answer=lowest_location.unwrap());

    lowest_location = None;

    for chunk in seeds.chunks(2) {
        let &[mut seed, length] = chunk else { panic!("uneven chunks") };
        let end = seed + length;
        while seed < end {
            let range = flat_map.range_for(seed);
            let value = range.map_value(seed).unwrap();
            if lowest_location.is_none() || lowest_location.unwrap() > value {
                lowest_location = Some(value);
            }
            // This is the lowest value we're going to get within this range, so jump ahead to the
            // end of it
            seed += range.end() - seed;
        }
    }

    info!(day=5, part=2, answer=lowest_location.unwrap());

    Ok(())
}
