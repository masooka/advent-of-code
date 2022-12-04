use std::io::{self, BufRead};
use std::str::FromStr;

use anyhow::Result;

/// An inclusive interval.
struct Interval {
    start: u32,
    end: u32,
}

impl Interval {
    /// Returns `true` if this interval contains the given interval.
    fn includes(&self, other: &Interval) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    /// Returns `true` if this interval overlaps the given interval.
    fn overlaps(&self, other: &Interval) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

impl FromStr for Interval {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.splitn(2, '-');
        let start = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("missing start"))?
            .parse()?;
        let end = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("missing end"))?
            .parse()?;
        Ok(Interval { start, end })
    }
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let lines = io::BufReader::new(stdin).lines();

    let mut include_count = 0;
    let mut overlap_count = 0;
    for line in lines {
        let line = line?;
        let mut parts = line.splitn(2, ',');
        let first = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("missing first"))?
            .parse::<Interval>()?;
        let second = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("missing second"))?
            .parse::<Interval>()?;
        if first.includes(&second) || second.includes(&first) {
            include_count += 1;
        }
        if first.overlaps(&second) {
            overlap_count += 1;
        }
    }

    println!("Part 1: {}", include_count);
    println!("Part 2: {}", overlap_count);
    Ok(())
}
