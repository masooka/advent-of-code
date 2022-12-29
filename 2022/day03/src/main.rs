use std::io::{self, Read};

use anyhow::{bail, Result};
use bit_set::BitSet;
use itertools::process_results;

struct Compartment {
    items: BitSet,
}

impl Compartment {
    fn new() -> Self {
        Self {
            items: BitSet::new(),
        }
    }

    /// Packs the items into the compartment.
    fn pack(&mut self, items: &[u8]) -> Result<()> {
        for &item in items {
            match item {
                b'a'..=b'z' => {
                    let index = item as usize - 'a' as usize + 1;
                    self.items.insert(index);
                }
                b'A'..=b'Z' => {
                    let index = item as usize - 'A' as usize + 27;
                    self.items.insert(index);
                }
                _ => bail!("invalid item: {}", item),
            }
        }
        Ok(())
    }
}

struct Rucksack {
    left: Compartment,
    right: Compartment,
}

impl Rucksack {
    /// Packs the given items into the rucksack.
    fn pack(items: &str) -> Result<Self> {
        let items = items.as_bytes();
        let (left_items, right_items) = items.split_at(items.len() / 2);
        let mut left = Compartment::new();
        left.pack(left_items)?;
        let mut right = Compartment::new();
        right.pack(right_items)?;
        Ok(Self { left, right })
    }

    /// Finds the first item in the both compartments.
    fn misplaced(&self) -> Option<usize> {
        self.left.items.intersection(&self.right.items).next()
    }

    fn union(&self) -> BitSet {
        let mut union = self.left.items.clone();
        union.union_with(&self.right.items);
        union
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

fn parse(input: &str) -> impl Iterator<Item = Result<Rucksack>> + '_ {
    input.lines().map(Rucksack::pack)
}

fn part1(input: &str) -> Result<usize> {
    parse(input)
        .map(|r| r.map(|r| r.misplaced().unwrap_or(0)))
        .sum()
}

fn part2(input: &str) -> Result<usize> {
    process_results(parse(input), |iter| {
        iter.enumerate()
            .fold(
                (0, BitSet::new()),
                |(mut badge_sum, mut common_items), (group, r)| {
                    let combined = r.union();
                    if group % 3 == 0 {
                        common_items = combined;
                    } else {
                        common_items.intersect_with(&combined);
                        if group % 3 == 2 {
                            let badge = common_items.iter().next().expect("input should be valid");
                            badge_sum += badge;
                        }
                    }
                    (badge_sum, common_items)
                },
            )
            .0
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
        assert_eq!(part1(INPUT).unwrap(), 157);
        assert_eq!(part2(INPUT).unwrap(), 70);
    }
}
