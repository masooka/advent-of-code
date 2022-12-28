use std::{
    cmp::Reverse,
    io::{self, Read},
};

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

fn parse(input: &str) -> impl Iterator<Item = usize> + '_ {
    input
        .lines()
        .map(|line| line.parse::<usize>().ok())
        .batching(|it| it.map_while(|x| x).sum1())
}

fn part1(input: &str) -> usize {
    parse(input).max().unwrap_or_default()
}

fn part2(input: &str) -> usize {
    parse(input)
        .map(Reverse)
        .k_smallest(3)
        .map(|value| value.0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn example() {
        assert_eq!(part1(INPUT), 24000);
        assert_eq!(part2(INPUT), 45000);
    }
}
