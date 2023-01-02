use std::{
    io::{self, Read},
    str,
};

use anyhow::Result;

struct Map {
    map: Vec<Vec<u8>>,
}

impl Map {
    fn height(&self) -> usize {
        self.map.len()
    }

    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        let height = self.map[y][x];
        (0..x).all(|i| self.map[y][i] < height)
            || (x + 1..self.width()).all(|i| self.map[y][i] < height)
            || (0..y).all(|i| self.map[i][x] < height)
            || (y + 1..self.height()).all(|i| self.map[i][x] < height)
    }

    fn count_visible(&self) -> usize {
        (0..self.height())
            .flat_map(|y| (0..self.width()).map(move |x| (x, y)))
            .filter(|&(x, y)| self.is_visible(x, y))
            .count()
    }

    fn score(&self, x: usize, y: usize) -> usize {
        let height = self.map[y][x];
        let mut score = 1;

        let mut distance = 0;
        for row in self.map.iter().take(y).rev() {
            distance += 1;
            if row[x] >= height {
                break;
            }
        }
        score *= distance;

        let mut distance = 0;
        for row in self.map.iter().skip(y + 1) {
            distance += 1;
            if row[x] >= height {
                break;
            }
        }
        score *= distance;

        let mut distance = 0;
        for cell in self.map[y].iter().take(x).rev() {
            distance += 1;
            if *cell >= height {
                break;
            }
        }
        score *= distance;

        let mut distance = 0;
        for cell in self.map[y].iter().skip(x + 1) {
            distance += 1;
            if *cell >= height {
                break;
            }
        }
        score *= distance;

        score
    }

    fn max_score(&self) -> usize {
        (0..self.height())
            .flat_map(|y| (0..self.width()).map(move |x| (x, y)))
            .map(|(x, y)| self.score(x, y))
            .max()
            .unwrap()
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

fn parse(input: &str) -> Map {
    let map = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();
    Map { map }
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    map.count_visible()
}

fn part2(input: &str) -> usize {
    let map = parse(input);
    map.max_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        const INPUT: &str = "30373
25512
65332
33549
35390
";
        assert_eq!(part1(INPUT), 21);
        assert_eq!(part2(INPUT), 8);
    }
}
