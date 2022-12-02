use std::io::{self, BufRead};

use anyhow::{bail, Result};

/// An outcome of a single round of the game.
///
/// * 0: draw
/// * 1: win
/// * 2: loss
#[derive(Clone, Copy)]
struct Outcome(u8);

impl TryFrom<char> for Outcome {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        let outcome = match value {
            'X' => 2,
            'Y' => 0,
            'Z' => 1,
            _ => bail!("invalid outcome: {}", value),
        };

        Ok(Outcome(outcome))
    }
}

/// A choice from rock (0), paper (1), scissors (2).
#[derive(Clone, Copy)]
struct Choice(u8);

impl Choice {
    /// Returns the choice to make to achieve the goal.
    fn plan(self, goal: Outcome) -> Choice {
        Choice((self.0 + goal.0) % 3)
    }
}

impl TryFrom<char> for Choice {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            'A' | 'X' => Ok(Choice(0)),
            'B' | 'Y' => Ok(Choice(1)),
            'C' | 'Z' => Ok(Choice(2)),
            _ => bail!("invalid choice: {}", value),
        }
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
        let outcome_score = match (self.player.0 + 3 - self.opponent.0) % 3 {
            0 => 3,
            1 => 6,
            2 => 0,
            _ => unreachable!(),
        };
        let selection_score = self.player.0 as usize + 1;
        outcome_score + selection_score
    }
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let lines = io::BufReader::new(stdin).lines();

    let mut score1 = 0;
    let mut score2 = 0;
    for line in lines {
        let line = line?;
        let mut chars = line.chars();
        let opponent = chars.next().unwrap().try_into()?;
        let second_col = chars.next_back().unwrap();
        let round = Round {
            opponent,
            player: second_col.try_into()?,
        };
        score1 += round.score();

        let round = Round {
            opponent,
            player: opponent.plan(second_col.try_into()?),
        };
        score2 += round.score();
    }

    println!("Part 1: {}", score1);
    println!("Part 2: {}", score2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        let round = Round {
            player: 'Y'.try_into().unwrap(),
            opponent: 'A'.try_into().unwrap(),
        };
        assert_eq!(round.score(), 8);

        let round = Round {
            player: 'X'.try_into().unwrap(),
            opponent: 'B'.try_into().unwrap(),
        };
        assert_eq!(round.score(), 1);

        let round = Round {
            player: 'Z'.try_into().unwrap(),
            opponent: 'C'.try_into().unwrap(),
        };
        assert_eq!(round.score(), 6);
    }
}
