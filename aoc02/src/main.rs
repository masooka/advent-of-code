use std::io::{self, Read};

use anyhow::{anyhow, bail, Error, Result};
use itertools::process_results;
use nom::{
    character::complete::{self, one_of},
    sequence::separated_pair,
    IResult,
};
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// An outcome of a single round of the game.
#[derive(Clone, Copy, IntoPrimitive)]
#[repr(u8)]
enum Outcome {
    Draw = 0,
    Win = 1,
    Loss = 2,
}

impl TryFrom<char> for Outcome {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        Ok(match value {
            'X' => Outcome::Loss,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => bail!("invalid outcome: {}", value),
        })
    }
}

/// A choice from rock, paper, scissors.
#[derive(Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
enum Choice {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl Choice {
    /// Returns the choice to make to achieve the goal.
    fn plan(self, goal: Outcome) -> Choice {
        ((u8::from(self) + u8::from(goal)) % 3).try_into().unwrap()
    }
}

impl TryFrom<char> for Choice {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        Ok(match value {
            'A' => Choice::Rock,
            'B' => Choice::Paper,
            'C' => Choice::Scissors,
            _ => bail!("invalid choice: {}", value),
        })
    }
}

/// A single round of Rock, Paper, Scissors.
struct Round {
    /// The opponent's choice.
    opponent: Choice,
    /// The player's choice.
    player: Choice,
}

impl Round {
    /// Returns the score for the player.
    fn score(&self) -> usize {
        let outcome_score = match (u8::from(self.player) + 3 - u8::from(self.opponent)) % 3 {
            0 => 3,
            1 => 6,
            2 => 0,
            _ => unreachable!(),
        };
        let selection_score = u8::from(self.player) as usize + 1;
        outcome_score + selection_score
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

fn parse(input: &str) -> impl Iterator<Item = Result<(char, char)>> + '_ {
    input.lines().map(|line| {
        let (input, (opponent, player)) = strategy_guide(line)
            .map_err(|e| anyhow!("failed to parse line: \"{}\": {}", line, e))?;
        if !input.is_empty() {
            bail!(
                "failed to parse line: \"{}\": leftover input: {}",
                line,
                input
            );
        }
        Ok((opponent, player))
    })
}

fn strategy_guide(input: &str) -> IResult<&str, (char, char)> {
    separated_pair(one_of("ABC"), complete::char(' '), one_of("XYZ"))(input)
}

fn part1(input: &str) -> Result<usize> {
    process_results(parse(input), |iter| {
        iter.map(|(opponent, player)| {
            let player =
                char::from_u32(u32::from(player) + u32::from('A') - u32::from('X')).unwrap();
            Round {
                opponent: opponent.try_into().unwrap(),
                player: player.try_into().unwrap(),
            }
            .score()
        })
        .sum()
    })
}

fn part2(input: &str) -> Result<usize> {
    process_results(parse(input), |iter| {
        iter.map(|(opponent, goal)| {
            let opponent = opponent.try_into().unwrap();
            let goal = goal.try_into().unwrap();
            Round {
                opponent,
                player: opponent.plan(goal),
            }
            .score()
        })
        .sum()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        const INPUT: &str = "\
A Y
B X
C Z
";
        assert_eq!(part1(INPUT).unwrap(), 15);
        assert_eq!(part2(INPUT).unwrap(), 12);
    }
}
