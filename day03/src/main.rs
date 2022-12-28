use std::io::{self, BufRead};

use anyhow::{bail, Result};
use bit_set::BitSet;

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
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let lines = io::BufReader::new(stdin).lines();

    let mut sum = 0;
    let mut badge_sum = 0;
    let mut common_items = BitSet::new();
    for (group, line) in lines.enumerate() {
        let line = line?;
        let rucksack = Rucksack::pack(&line)?;
        let misplaced = rucksack.misplaced();
        sum += misplaced.unwrap_or(0);

        let mut combined = rucksack.left.items.clone();
        combined.union_with(&rucksack.right.items);
        if group % 3 == 0 {
            common_items = combined;
        } else {
            common_items.intersect_with(&combined);
            if group % 3 == 2 {
                let badge = common_items
                    .iter()
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("no badge found"))?;
                println!("Group {}: {}", group, badge);
                badge_sum += badge;
            }
        }
    }

    println!("Part 1: {}", sum);
    println!("Part 2: {}", badge_sum);
    Ok(())
}
