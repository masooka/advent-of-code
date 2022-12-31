use std::io::{self, Read};

use anyhow::{anyhow, Result};
use nom::bytes::complete::take_while1;
use nom::character::complete::{self, not_line_ending};
use nom::combinator::{all_consuming, eof, map, map_res};
use nom::multi::{fold_many0, many0};
use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};
use nom::IResult;
use nom::{branch::alt, bytes::complete::tag, character::complete::digit1};

#[derive(Debug)]
enum Entry<'n> {
    Dir(DirEntry<'n>),
    File(FileEntry<'n>),
}

impl Entry<'_> {
    fn size(&self) -> usize {
        match self {
            Entry::Dir(dir) => dir.size,
            Entry::File(file) => file.size,
        }
    }
}

#[derive(Debug)]
struct DirEntry<'n> {
    #[allow(dead_code)]
    name: &'n str,
    entries: Vec<Entry<'n>>,
    size: usize,
}

impl DirEntry<'_> {
    fn dir_fold<F>(&self, acc: usize, func: F) -> usize
    where
        F: Fn(usize, &DirEntry) -> usize + Copy,
    {
        let mut acc = acc;
        for dir in self.entries.iter().filter_map(|entry| match entry {
            Entry::Dir(dir) => Some(dir),
            Entry::File(_) => None,
        }) {
            acc = dir.dir_fold(acc, func);
        }
        func(acc, self)
    }
}

#[derive(Debug)]
struct FileEntry<'n> {
    #[allow(dead_code)]
    name: &'n str,
    size: usize,
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

fn parse(input: &str) -> Result<DirEntry> {
    let (_, dir) = all_consuming(cd)(input).map_err(|e| anyhow!("{}", e))?;
    Ok(dir)
}

fn cd(input: &str) -> IResult<&str, DirEntry> {
    map(
        tuple((
            delimited(tag("$ cd "), dir_name, complete::char('\n')),
            ls,
            many0(terminated(cd, alt((tag("$ cd ..\n"), eof)))),
        )),
        |(name, files, dirs)| {
            let entries = files
                .into_iter()
                .map(Entry::File)
                .chain(dirs.into_iter().map(Entry::Dir))
                .collect::<Vec<_>>();
            let size = entries.iter().map(Entry::size).sum();
            DirEntry {
                name,
                entries,
                size,
            }
        },
    )(input)
}

fn dir_name(input: &str) -> IResult<&str, &str> {
    let (input, name) = take_while1(|c| "abcdefghijklmnopqrstuvwxyz/".contains(c))(input)?;
    Ok((input, name))
}

fn ls(input: &str) -> IResult<&str, Vec<FileEntry>> {
    preceded(
        tag("$ ls\n"),
        fold_many0(entry, Vec::new, |mut acc, item| {
            if let Some(item) = item {
                acc.push(item);
            }
            acc
        }),
    )(input)
}

fn entry(input: &str) -> IResult<&str, Option<FileEntry>> {
    alt((map(file_entry, Some), map(dir_entry, |_| None)))(input)
}

fn dir_entry(input: &str) -> IResult<&str, &str> {
    delimited(tag("dir "), dir_name, complete::char('\n'))(input)
}

fn file_entry(input: &str) -> IResult<&str, FileEntry> {
    terminated(
        map(
            separated_pair(
                map_res(digit1, str::parse),
                complete::char(' '),
                not_line_ending,
            ),
            |(size, name)| FileEntry { name, size },
        ),
        complete::char('\n'),
    )(input)
}

fn part1(input: &str) -> Result<usize> {
    let root = parse(input)?;
    let sum = root.dir_fold(0, |mut acc, dir| {
        if dir.size <= 100_000 {
            acc += dir.size;
        }
        acc
    });
    Ok(sum)
}

fn part2(input: &str) -> Result<usize> {
    let root = parse(input)?;
    let target = 30_000_000 - (70_000_000 - root.size);
    let min = root.dir_fold(usize::MAX, |acc, dir| {
        if dir.size >= target {
            dir.size.min(acc)
        } else {
            acc
        }
    });
    Ok(min)
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
        assert_eq!(part1(INPUT).unwrap(), 95_437);
        assert_eq!(part2(INPUT).unwrap(), 24_933_642);
    }
}
