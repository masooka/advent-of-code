use std::collections::HashMap;
use std::io;
use std::path::PathBuf;

use anyhow::Result;

fn main() -> Result<()> {
    let lines = io::read_to_string(io::stdin())?
        .split('\n')
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    let mut cur = PathBuf::new();
    let mut size = 0;
    let iter = lines.iter();
    let mut sizes = HashMap::<PathBuf, usize>::new();
    for line in iter {
        let tokens = line.split(' ').collect::<Vec<_>>();
        if tokens[0] == "$" {
            if tokens[1] == "cd" {
                *sizes.entry(cur.clone()).or_default() += size;
                size = 0;
                if tokens[2] == ".." {
                    cur.pop();
                } else {
                    cur.push(tokens[2]);
                }
            } else {
                continue;
            }
        } else if let Ok(s) = tokens[0].parse::<usize>() {
            size += s;
        }
    }
    *sizes.entry(cur.clone()).or_default() += size;

    let mut tree_sizes = sizes.clone();
    for (path, size) in sizes {
        let mut cur = path;
        while let Some(parent) = cur.parent() {
            *tree_sizes.entry(parent.to_path_buf()).or_default() += size;
            cur = parent.to_path_buf();
        }
    }

    let mut sum = 0;
    for &size in tree_sizes.values() {
        if size <= 100_000 {
            sum += size;
        }
    }
    println!("Part 1: {}", sum);

    let target = 30_000_000 - (70_000_000 - tree_sizes[&PathBuf::from("/")]);
    let min = tree_sizes
        .values()
        .filter(|&size| size >= &target)
        .min()
        .unwrap();
    println!("Part 2: {}", min);
    Ok(())
}
