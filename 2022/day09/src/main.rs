use std::{
    collections::HashSet,
    io::{self, Read},
};

use anyhow::{anyhow, Result};
use nom::{
    character::complete::{self, digit1, one_of},
    combinator::{all_consuming, map_res},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone, Copy)]
enum Motion {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

struct Rope<const N: usize> {
    coords: [(i32, i32); N],
    tail_visits: HashSet<(i32, i32)>,
}

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        Self {
            coords: [(0, 0); N],
            tail_visits: HashSet::new(),
        }
    }

    fn move_rope(&mut self, motion: Motion) {
        let ((dx, dy), distance) = match motion {
            Motion::Up(distance) => ((0, 1), distance),
            Motion::Down(distance) => ((0, -1), distance),
            Motion::Left(distance) => ((-1, 0), distance),
            Motion::Right(distance) => ((1, 0), distance),
        };

        for _ in 0..distance {
            let (x, y) = self.coords[0];
            self.coords[0] = (x + dx, y + dy);
            for i in 0..N - 1 {
                self.coords[i + 1] = Self::follow(self.coords[i], self.coords[i + 1]);
            }
            self.tail_visits.insert(self.coords[N - 1]);
        }
    }

    fn follow(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
        if head.0 == tail.0 - 2 {
            if head.1 == tail.1 - 2 {
                (tail.0 - 1, tail.1 - 1)
            } else if head.1 == tail.1 + 2 {
                (tail.0 - 1, tail.1 + 1)
            } else {
                (tail.0 - 1, head.1)
            }
        } else if head.0 == tail.0 + 2 {
            if head.1 == tail.1 - 2 {
                (tail.0 + 1, tail.1 - 1)
            } else if head.1 == tail.1 + 2 {
                (tail.0 + 1, tail.1 + 1)
            } else {
                (tail.0 + 1, head.1)
            }
        } else if head.1 == tail.1 - 2 {
            if head.0 == tail.0 - 2 {
                (tail.0 - 1, tail.1 - 1)
            } else if head.0 == tail.0 + 2 {
                (tail.0 + 1, tail.1 - 1)
            } else {
                (head.0, tail.1 - 1)
            }
        } else if head.1 == tail.1 + 2 {
            if head.0 == tail.0 - 2 {
                (tail.0 - 1, tail.1 + 1)
            } else if head.0 == tail.0 + 2 {
                (tail.0 + 1, tail.1 + 1)
            } else {
                (head.0, tail.1 + 1)
            }
        } else {
            tail
        }
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

fn parse(input: &str) -> Result<Vec<Motion>> {
    let mut motions = Vec::new();
    for line in input.lines() {
        let motion = all_consuming(motion)(line)
            .map(|(_, motion)| motion)
            .map_err(|_| anyhow!("Failed to parse line: {line}"))?;
        motions.push(motion);
    }
    Ok(motions)
}

fn motion(input: &str) -> IResult<&str, Motion> {
    separated_pair(
        one_of("UDLR"),
        complete::char(' '),
        map_res(digit1, str::parse::<i32>),
    )(input)
    .map(|(input, (direction, distance))| {
        (
            input,
            match direction {
                'U' => Motion::Up(distance),
                'D' => Motion::Down(distance),
                'L' => Motion::Left(distance),
                'R' => Motion::Right(distance),
                _ => unreachable!(),
            },
        )
    })
}

fn part1(input: &str) -> Result<usize> {
    parse(input).map(|motion| count_tail_visits::<2>(&motion))
}

fn part2(input: &str) -> Result<usize> {
    parse(input).map(|motions| count_tail_visits::<10>(&motions))
}

fn count_tail_visits<const N: usize>(motions: &[Motion]) -> usize {
    let mut rope = Rope::<N>::new();
    for motion in motions {
        rope.move_rope(*motion);
    }
    rope.tail_visits.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        const INPUT1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";
        const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
        assert_eq!(part1(INPUT1).unwrap(), 13);
        assert_eq!(part2(INPUT1).unwrap(), 1);
        assert_eq!(part2(INPUT2).unwrap(), 36);
    }
}
