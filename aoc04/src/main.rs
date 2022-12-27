use std::{
    io::{self, Read},
    ops::RangeInclusive,
    str::FromStr,
};

use anyhow::{anyhow, Result};
use itertools::process_results;
use nom::{character::complete, combinator::map_res, sequence::separated_pair, IResult};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

fn parse<Idx: FromStr>(
    input: &str,
) -> impl Iterator<Item = Result<(RangeInclusive<Idx>, RangeInclusive<Idx>)>> + '_ {
    input.lines().map(|line| {
        let (input, (start, end)) = assignment_pair(line)
            .map_err(|e| anyhow!("failed to parse line \"{}\": {}", line, e))?;
        if !input.is_empty() {
            return Err(anyhow!("unexpected input \"{}\"", input));
        }
        Ok((start, end))
    })
}

fn assignment_pair<Idx: FromStr>(
    input: &str,
) -> IResult<&str, (RangeInclusive<Idx>, RangeInclusive<Idx>)> {
    separated_pair(interval, complete::char(','), interval)(input)
}

fn interval<Idx: FromStr>(input: &str) -> IResult<&str, RangeInclusive<Idx>> {
    let (input, (start, end)) = separated_pair(
        map_res(complete::digit1, str::parse),
        complete::char('-'),
        map_res(complete::digit1, str::parse),
    )(input)?;
    Ok((input, RangeInclusive::new(start, end)))
}

fn part1(input: &str) -> Result<usize> {
    process_results(parse::<u64>(input), |iter| {
        iter.filter(|(a, b)| includes(a, b) || includes(b, a))
            .count()
    })
}

fn part2(input: &str) -> Result<usize> {
    process_results(parse::<u64>(input), |iter| {
        iter.filter(|(a, b)| overlaps(a, b)).count()
    })
}

/// Returns `true` if range `a` includes range `b`.
fn includes<Idx: PartialOrd>(a: &RangeInclusive<Idx>, b: &RangeInclusive<Idx>) -> bool {
    a.start() <= b.start() && b.end() <= a.end()
}

/// Returns `true` if range `a` overlaps range `b`.
fn overlaps<Idx: PartialOrd>(a: &RangeInclusive<Idx>, b: &RangeInclusive<Idx>) -> bool {
    a.start() <= b.end() && b.start() <= a.end()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        const INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
        assert_eq!(part1(INPUT).unwrap(), 2);
        assert_eq!(part2(INPUT).unwrap(), 4);
    }
}
