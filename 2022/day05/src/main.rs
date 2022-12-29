use std::{
    io::{self, Read},
    mem,
};

use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, digit1, one_of},
    combinator::{all_consuming, map_res, peek},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
struct RearrangementStep {
    num: usize,
    from: usize,
    to: usize,
}

struct CrateStacks {
    stacks: Vec<Vec<char>>,
}

impl CrateStacks {
    fn new(stacks: Vec<Vec<char>>) -> Self {
        Self { stacks }
    }

    fn top_crates(&self) -> Vec<char> {
        self.stacks
            .iter()
            .map(|s| s.last().copied().unwrap_or('.'))
            .collect()
    }

    fn move_crate(&mut self, from: usize, to: usize) -> Result<()> {
        let crate_ = self.stacks[from].pop().ok_or_else(|| {
            anyhow!(
                "Tried to move crate from empty stack {} to stack {}",
                from,
                to
            )
        })?;
        self.stacks[to].push(crate_);
        Ok(())
    }

    fn move_multiple_crates(&mut self, from: usize, to: usize, num: usize) {
        let from_stack = mem::take(&mut self.stacks[from]);
        self.stacks[to].extend(from_stack[from_stack.len() - num..].iter());
        drop(mem::replace(&mut self.stacks[from], from_stack));
        let new_len = self.stacks[from].len() - num;
        self.stacks[from].truncate(new_len);
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

fn parse(
    input: &str,
) -> Result<(
    CrateStacks,
    impl Iterator<Item = Result<RearrangementStep>> + '_,
)> {
    let mut lines = input.lines();
    let mut stacks = lines
        .by_ref()
        .map_while(|line| stacks_row(line).ok().map(|(_, row)| row))
        .fold(Vec::new(), |mut stacks, row| {
            while row.len() > stacks.len() {
                stacks.push(Vec::new());
            }
            for (&sym, stack) in row.iter().zip(stacks.iter_mut()) {
                if sym == ' ' {
                    continue;
                }
                stack.push(sym);
            }
            stacks
        });
    stacks.iter_mut().for_each(|stack| stack.reverse());
    lines
        .next()
        .filter(|line| line.is_empty())
        .ok_or_else(|| anyhow!("missing empty line"))?;

    let iter = lines.map(|line| {
        rearrangement_step(line)
            .map(|(_, step)| step)
            .map_err(|e| anyhow!("failed to parse line: \"{}\": {}", line, e))
    });
    Ok((CrateStacks::new(stacks), iter))
}

fn stacks_row(input: &str) -> IResult<&str, Vec<char>> {
    all_consuming(separated_list1(tag(" "), alt((stack_elem, stack_empty))))(input)
}

fn stack_elem(input: &str) -> IResult<&str, char> {
    let (input, (_, elem, _)) =
        tuple((complete::char('['), complete::anychar, complete::char(']')))(input)?;
    Ok((input, elem))
}

fn stack_empty(input: &str) -> IResult<&str, char> {
    let (input, _) = tag("   ")(input)?;
    Ok((input, ' '))
}

fn rearrangement_step(input: &str) -> IResult<&str, RearrangementStep> {
    let (input, (num, from, to)) = all_consuming(tuple((
        preceded(tag("move "), map_res(digit1, str::parse)),
        preceded(
            tag(" from "),
            map_res(preceded(peek(one_of("123456789")), digit1), |s: &str| {
                s.parse::<usize>().map(|n| n - 1)
            }),
        ),
        preceded(
            tag(" to "),
            map_res(preceded(peek(one_of("123456789")), digit1), |s: &str| {
                s.parse::<usize>().map(|n| n - 1)
            }),
        ),
    )))(input)?;
    Ok((input, RearrangementStep { num, from, to }))
}

fn part1(input: &str) -> Result<String> {
    let (mut stacks, iter) = parse(input)?;
    for step in iter {
        let step = step?;
        let mut remaining = step.num;
        while remaining > 0 {
            stacks.move_crate(step.from, step.to)?;
            remaining -= 1;
        }
    }
    Ok(stacks.top_crates().into_iter().collect())
}

fn part2(input: &str) -> Result<String> {
    let (mut stacks, iter) = parse(input)?;
    for step in iter {
        let step = step?;
        stacks.move_multiple_crates(step.from, step.to, step.num);
    }
    Ok(stacks.top_crates().into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
        assert_eq!(part1(INPUT).unwrap(), "CMZ");
        assert_eq!(part2(INPUT).unwrap(), "MCD");
    }
}
