use color_eyre::Report;
use fnv::FnvHashMap;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{newline, one_of};
use nom::combinator::all_consuming;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{terminated, tuple};
use tracing::{debug, info};

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    one_of("LR")(input).map(|(rest, ch)|
        (rest,
            match ch {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Unexpected direction: {ch}")
            }
        )
    )
}

fn parse_node(input: &str) -> IResult<&str, (&str, &str, &str)> {
    tuple((
        take_while1(|c: char| c.is_alphanumeric()),
        tag(" = ("),
        take_while1(|c: char| c.is_alphanumeric()),
        tag(", "),
        take_while1(|c: char| c.is_alphanumeric()),
        tag(")")
    ))(input).map(|(rest, (origin, _, dest1, _, dest2, _))|
        (rest, (origin, dest1, dest2))
    )
}

fn parse_nodeset(input: &str) -> IResult<&str, FnvHashMap<String, (String, String)>> {
    many1(
        terminated(parse_node, newline)
    )(input).map(|(rest, nodes)| {
        let mut all_nodes = FnvHashMap::default();
        for (origin, left, right) in nodes {
            all_nodes.insert(origin.to_string(), (left.to_string(), right.to_string()));
        }
        (rest, all_nodes)
    })
}

fn walk(nodeset: &FnvHashMap<String, (String, String)>, directions: &[Direction], start: &String, is_end: impl Fn(&String) -> bool) -> (String, usize) {
    let mut steps = directions.iter().cycle();
    let mut position = start;
    let mut steps_taken = 0;
    loop {
        let node = nodeset.get(position).unwrap();
        match steps.next().unwrap() {
            Direction::Left => position = &node.0,
            Direction::Right => position = &node.1,
        }
        steps_taken += 1;
        if is_end(position) {
            return (position.to_string(), steps_taken)
        }
    }
}

fn prime_factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut factor = 2;
    let mut factors = Vec::new();
    while n > 1 {
        while n % factor == 0 {
            factors.push(factor);
            n /= factor;
        }
        factor += 1;
    }
    factors
}

pub(crate) fn solve(input: String) -> Result<(), Report> {

    let (directions, nodeset) = all_consuming(tuple((
        terminated(
            many1(parse_direction),
            newline,
        ),
        newline,
        parse_nodeset
    )))(&input).map(|(_, (directions, _, nodeset))| (directions, nodeset)).unwrap();

    debug!(directions=?directions, nodeset=?nodeset);

    if nodeset.contains_key("AAA") {
        let mut steps = directions.iter().cycle();
        let mut position = "AAA";
        let mut steps_taken = 0;
        while position != "ZZZ" {
            let node = nodeset.get(position).unwrap();
            match steps.next().unwrap() {
                Direction::Left => position = &node.0,
                Direction::Right => position = &node.1,
            }
            steps_taken += 1;
        }

        info!(day=8, part=1, answer=steps_taken);
    } else {
        info!("Skip part1, no 'AAA'")
    }

    let positions: Vec<&String> = nodeset.keys().filter(|k| k.ends_with('A')).collect();
    debug!(?positions);

    let mut step_counts = Vec::new();

    for pos in positions {
        let (end, initial_steps) = walk(&nodeset, &directions, pos, |s| s.ends_with('Z'));
        let (end2, loop_steps) = walk(&nodeset, &directions, &end, |s| s == &end);
        assert_eq!(end, end2);
        debug!(pos, initial_steps, loop_steps);

        // For my input data, all the loop steps turned out to be the same as the initial steps
        step_counts.push(loop_steps);
    }

    // Figure out the least common multiple of all the step counts
    // https://en.wikipedia.org/wiki/Least_common_multiple
    // We have to make a prime factor soup that has as many of each prime factor as any of our
    // input numbers has.

    let mut prime_factor_soup = Vec::new();
    for step_count in step_counts {
        let factors = prime_factors(step_count as u64);
        for factor in &factors {
            // How many instances of this factor do we have in the soup?
            let have = prime_factor_soup.iter().filter(|&pfs| pfs == factor).count();
            // How many instances of this factor do we need in the soup?  At least as many as our
            // current step_count has.
            let need = factors.iter().filter(|&f| f == factor).count();
            for _ in have..need {
                // put it in the soup
                prime_factor_soup.push(*factor);
            }
        }
    }
    debug!(?prime_factor_soup);

    let product: u64 = prime_factor_soup.into_iter().product();

    info!(day=8, part=2, answer=product);
    Ok(())
}
