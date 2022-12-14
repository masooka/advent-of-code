use std::{io, str::FromStr};

use anyhow::Result;

#[derive(Debug)]
struct Path {
    coords: Vec<(usize, usize)>,
}

impl FromStr for Path {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        s.split(" -> ")
            .map(|s| {
                let mut parts = s.split(',');
                let x = parts.next().unwrap().parse::<usize>()?;
                let y = parts.next().unwrap().parse::<usize>()?;
                Ok((x, y))
            })
            .collect::<Result<Vec<_>>>()
            .map(|coords| Path { coords })
    }
}

struct Cave {
    view: Vec<Vec<char>>,
    min_x: usize,
}

impl Cave {
    fn new(paths: &[Path], floor: bool) -> Self {
        let mut min_x = *paths
            .iter()
            .map(|p| p.coords.iter().map(|(x, _)| x).min().unwrap())
            .min()
            .unwrap();
        let max_x = *paths
            .iter()
            .map(|p| p.coords.iter().map(|(x, _)| x).max().unwrap())
            .max()
            .unwrap();
        let max_y = *paths
            .iter()
            .map(|p| p.coords.iter().map(|(_, y)| y).max().unwrap())
            .max()
            .unwrap();

        let mut view = if floor {
            vec![vec!['.'; max_y + 3]; max_y * 2 + 7]
        } else {
            vec![vec!['.'; max_y + 1]; max_x - min_x + 1]
        };
        if floor {
            min_x = 500 - max_y - 3;
            for col in &mut view {
                col[max_y + 2] = '#';
            }
        }

        for path in paths {
            for segment in path.coords.windows(2) {
                let s = segment[0];
                let e = segment[1];

                if s.0 == e.0 {
                    let (from, to) = if s.1 < e.1 { (s.1, e.1) } else { (e.1, s.1) };
                    for y in from..=to {
                        view[s.0 - min_x][y] = '#';
                    }
                } else {
                    let (from, to) = if s.0 < e.0 { (s.0, e.0) } else { (e.0, s.0) };
                    for x in from..=to {
                        view[x - min_x][s.1] = '#';
                    }
                }
            }
        }
        Cave { view, min_x }
    }

    fn drop_sand(&mut self, x: usize, y: usize) -> bool {
        let mut x = x;
        let mut y = y;
        let mut coord = (x, y);
        while y < self.view[0].len() {
            if self.view[x - self.min_x][y] == '.' {
                coord = (x, y);
                y += 1;
                continue;
            }
            if x == self.min_x {
                return false;
            }
            if self.view[x - self.min_x - 1][y] == '.' {
                x -= 1;
                continue;
            }
            if x == self.min_x + self.view.len() - 1 {
                return false;
            }
            if self.view[x - self.min_x + 1][y] == '.' {
                x += 1;
                continue;
            }
            self.view[coord.0 - self.min_x][coord.1] = 'o';
            return true;
        }
        true
    }

    #[allow(dead_code)]
    fn display(&self) {
        for i in 0..self.view[0].len() {
            for j in 0..self.view.len() {
                print!("{}", self.view[j][i]);
            }
            println!();
        }
    }
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let paths = input.lines().map(str::parse).collect::<Result<Vec<_>>>()?;

    let mut cave = Cave::new(&paths, false);
    let mut count = 0;
    while cave.drop_sand(500, 0) {
        count += 1;
    }
    println!("Part 1: {}", count);

    let mut cave = Cave::new(&paths, true);
    count = 0;
    while cave.drop_sand(500, 0) {
        count += 1;
        if cave.view[500 - cave.min_x][0] == 'o' {
            break;
        }
    }
    println!("Part 2: {}", count);

    Ok(())
}
