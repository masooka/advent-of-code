use std::io::{self, Read};

use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::{all_consuming, map},
    sequence::separated_pair,
    IResult,
};

struct Crt {
    pixels: Vec<bool>,
}

impl Crt {
    fn new() -> Self {
        Self {
            pixels: vec![false; 6 * 40],
        }
    }

    fn set(&mut self, cycle: usize) {
        self.pixels[(cycle - 1) % (40 * 6)] = true;
    }

    fn display(&self) {
        for y in 0..6 {
            for x in 0..40 {
                print!("{}", if self.pixels[y * 40 + x] { "#" } else { "." });
            }
            println!();
        }
    }
}

#[allow(clippy::cast_sign_loss)]
fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut x = 1;
    let mut cycle = 0;
    let sampling = vec![20, 60, 100, 140, 180, 220];
    let mut sampling_iter = sampling.iter();
    let mut sampling_cycle = sampling_iter.next().unwrap();
    let mut sum = 0;
    let mut crt = Crt::new();
    for instruction in parse(&input) {
        let instruction = instruction?;
        match instruction {
            Instruction::Addx(arg) => {
                cycle += 1;
                if visible(cycle, x) {
                    crt.set(cycle);
                }
                cycle += 1;
                if visible(cycle, x) {
                    crt.set(cycle);
                }
                if cycle >= *sampling_cycle {
                    sum += x as usize * *sampling_cycle;
                    if let Some(next) = sampling_iter.next() {
                        sampling_cycle = next;
                    } else {
                        sampling_cycle = &usize::MAX;
                    }
                }
                x += arg;
            }
            Instruction::Noop => {
                cycle += 1;
                if visible(cycle, x) {
                    crt.set(cycle);
                }
                if cycle >= *sampling_cycle {
                    sum += x as usize * *sampling_cycle;
                    if let Some(next) = sampling_iter.next() {
                        sampling_cycle = next;
                    } else {
                        sampling_cycle = &usize::MAX;
                    }
                }
            }
        }
    }

    println!("Part 1: {sum}");
    println!("Part 2:");
    crt.display();
    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Addx(i32),
    Noop,
}

fn parse(input: &str) -> impl Iterator<Item = Result<Instruction>> + '_ {
    input.lines().map(|line| {
        let (_, instruction) =
            all_consuming(instruction)(line).map_err(|s| anyhow!("Invalid instruction: {s}"))?;
        Ok(instruction)
    })
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(
            separated_pair(tag("addx"), complete::char(' '), complete::i32),
            |(_, arg)| Instruction::Addx(arg),
        ),
        map(tag("noop"), |_| Instruction::Noop),
    ))(input)
}

#[allow(clippy::cast_sign_loss)]
fn visible(cycle: usize, x: i32) -> bool {
    let pos = (cycle - 1) % 40;
    if x < 1 {
        pos <= x as usize + 1
    } else {
        x as usize - 1 <= pos && pos <= x as usize + 1
    }
}
