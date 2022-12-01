use std::io::{self, BufRead};

use anyhow::Result;

/// A storage for `N` largest values inserted. Initially all values are `0`.
struct Largest<const N: usize> {
    values: [usize; N],
}

impl<const N: usize> Largest<N> {
    fn new() -> Self {
        Self { values: [0; N] }
    }

    /// Inserts a new value into the storage, replacing the smallest value if
    /// necessary.
    fn insert(&mut self, value: usize) {
        let mut i = 0;
        while i < N && self.values[i] >= value {
            i += 1;
        }
        if i < N {
            self.values[i..].rotate_right(1);
            self.values[i] = value;
        }
    }

    /// Returns an iterator over the values in the descending order.
    fn iter(&self) -> impl Iterator<Item = &usize> {
        self.values.iter()
    }

    /// Returns the maximum value.
    fn max(&self) -> usize {
        self.values[0]
    }
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut lines = io::BufReader::new(stdin).lines();
    let mut calories = Largest::<3>::new();

    while let Some(line) = lines.by_ref().next() {
        let mut sum = line?.parse::<usize>()?;
        while let Some(line) = lines.by_ref().next() {
            let line = line?;
            if line.is_empty() {
                break;
            }
            sum += line.parse::<usize>()?;
        }
        calories.insert(sum);
    }

    println!(
        "The total calories carried by the elf carrying the most calories: {}",
        calories.max()
    );
    println!(
        "The total calories carried by the elves: {}",
        calories.iter().sum::<usize>()
    );
    Ok(())
}
