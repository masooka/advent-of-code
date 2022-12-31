use std::collections::HashMap;
use std::io::{self, Read};
use std::path::PathBuf;

use anyhow::Result;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

fn parse(input: &str) -> HashMap<PathBuf, usize> {
    let mut cur = PathBuf::new();
    let mut size = 0;
    let mut sizes = HashMap::<PathBuf, usize>::new();
    for line in input.lines() {
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

    tree_sizes
}

fn part1(input: &str) -> usize {
    let sizes = parse(input);
    sizes.values().filter(|&s| *s <= 100_000).sum()
}

fn part2(input: &str) -> usize {
    let sizes = parse(input);
    let target = 30_000_000 - (70_000_000 - sizes[&PathBuf::from("/")]);
    let min = sizes
        .values()
        .filter(|&size| size >= &target)
        .min()
        .unwrap();
    *min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        const INPUT: &str = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";
        assert_eq!(part1(INPUT), 95_437);
        assert_eq!(part2(INPUT), 24_933_642);
    }
}
