use std::{collections::HashSet, io, str::FromStr};

use anyhow::Result;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

impl Cube {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn neighbors(&self) -> impl Iterator<Item = Cube> + '_ {
        vec![
            Cube::new(self.x - 1, self.y, self.z),
            Cube::new(self.x + 1, self.y, self.z),
            Cube::new(self.x, self.y - 1, self.z),
            Cube::new(self.x, self.y + 1, self.z),
            Cube::new(self.x, self.y, self.z - 1),
            Cube::new(self.x, self.y, self.z + 1),
        ]
        .into_iter()
    }
}

impl FromStr for Cube {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let nums = s.split(',').collect::<Vec<_>>();
        Ok(Cube {
            x: nums[0].parse()?,
            y: nums[1].parse()?,
            z: nums[2].parse()?,
        })
    }
}

struct Lava {
    cubes: Vec<Cube>,
}

struct Space {
    x_range: std::ops::RangeInclusive<isize>,
    y_range: std::ops::RangeInclusive<isize>,
    z_range: std::ops::RangeInclusive<isize>,
}

impl Space {
    fn contains(&self, cube: &Cube) -> bool {
        self.x_range.contains(&cube.x)
            && self.y_range.contains(&cube.y)
            && self.z_range.contains(&cube.z)
    }
}

impl Lava {
    fn contains(&self, x: isize, y: isize, z: isize) -> bool {
        self.cubes
            .iter()
            .any(|cube| cube.x == x && cube.y == y && cube.z == z)
    }

    fn steam(&self) -> HashSet<(isize, isize, isize)> {
        let max_x = self.cubes.iter().map(|cube| cube.x).max().unwrap();
        let min_x = self.cubes.iter().map(|cube| cube.x).min().unwrap();
        let max_y = self.cubes.iter().map(|cube| cube.y).max().unwrap();
        let min_y = self.cubes.iter().map(|cube| cube.y).min().unwrap();
        let max_z = self.cubes.iter().map(|cube| cube.z).max().unwrap();
        let min_z = self.cubes.iter().map(|cube| cube.z).min().unwrap();

        let space = Space {
            x_range: min_x - 1..=max_x + 1,
            y_range: min_y - 1..=max_y + 1,
            z_range: min_z - 1..=max_z + 1,
        };
        let steam = Cube::new(min_x - 1, min_y - 1, min_z - 1);
        let mut visited = HashSet::new();
        let mut to_visit = vec![steam];
        while let Some(cur) = to_visit.pop() {
            if visited.contains(&(cur.x, cur.y, cur.z)) {
                continue;
            }
            visited.insert((cur.x, cur.y, cur.z));
            for next in cur.neighbors() {
                if !space.contains(&next) {
                    continue;
                }
                if self.contains(next.x, next.y, next.z) {
                    continue;
                }
                if visited.contains(&(next.x, next.y, next.z)) {
                    continue;
                }
                to_visit.push(next.clone());
            }
        }

        visited
    }
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let lines = input.lines().collect::<Vec<_>>();
    let cubes = lines
        .iter()
        .map(|&s| s.parse::<Cube>().unwrap())
        .collect::<Vec<_>>();
    let lava = Lava { cubes };

    let mut surface_area = 0;
    for cube in &lava.cubes {
        let mut sides = 6;
        if lava.contains(cube.x - 1, cube.y, cube.z) {
            sides -= 1;
        }
        if lava.contains(cube.x + 1, cube.y, cube.z) {
            sides -= 1;
        }
        if lava.contains(cube.x, cube.y - 1, cube.z) {
            sides -= 1;
        }
        if lava.contains(cube.x, cube.y + 1, cube.z) {
            sides -= 1;
        }
        if lava.contains(cube.x, cube.y, cube.z - 1) {
            sides -= 1;
        }
        if lava.contains(cube.x, cube.y, cube.z + 1) {
            sides -= 1;
        }

        surface_area += sides;
    }
    println!("Part 1: {surface_area}");

    let steam = lava.steam();
    let mut contacting_steam = 0;
    for cube in &lava.cubes {
        for neighbor in cube.neighbors() {
            if steam.contains(&(neighbor.x, neighbor.y, neighbor.z)) {
                contacting_steam += 1;
            }
        }
    }
    println!("Part 2: {contacting_steam}");

    Ok(())
}
