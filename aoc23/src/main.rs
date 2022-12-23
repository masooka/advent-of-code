use std::io;

use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Open,
    Elf,
}

#[derive(Clone)]
struct Board {
    tiles: Vec<Vec<Tile>>,
}

impl Board {
    fn new(rows: &[&str]) -> Self {
        const MARGIN: usize = 200;
        let mut tiles = Vec::new();
        let mid = rows
            .iter()
            .map(|row| {
                let center = row
                    .chars()
                    .map(|c| match c {
                        '.' => Tile::Open,
                        '#' => Tile::Elf,
                        _ => panic!("invalid tile"),
                    })
                    .collect::<Vec<_>>();
                let mut full = Vec::new();
                full.extend((0..MARGIN).map(|_| Tile::Open));
                full.extend(center);
                full.extend((0..MARGIN).map(|_| Tile::Open));
                full
            })
            .collect::<Vec<_>>();
        let width = mid[0].len();
        tiles.extend((0..MARGIN).map(|_| vec![Tile::Open; width]));
        tiles.extend(mid);
        tiles.extend((0..MARGIN).map(|_| vec![Tile::Open; width]));
        Self { tiles }
    }

    fn propose(&self, round: usize) -> Option<Vec<((usize, usize), (usize, usize))>> {
        let mut proposals = Vec::new();
        let mut moving = 0;
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == Tile::Elf {
                    if let Some(pos) = self.elf_move(round, y, x) {
                        proposals.push((pos, (y, x)));
                        moving += 1;
                    } else {
                        proposals.push(((y, x), (y, x)));
                    }
                }
            }
        }
        if moving > 0 {
            Some(proposals)
        } else {
            None
        }
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    fn elf_move(&self, round: usize, y: usize, x: usize) -> Option<(usize, usize)> {
        const DIRECTIONS: [[(isize, isize); 3]; 4] = [
            [(-1, -1), (-1, 0), (-1, 1)],
            [(1, -1), (1, 0), (1, 1)],
            [(-1, -1), (0, -1), (1, -1)],
            [(-1, 1), (0, 1), (1, 1)],
        ];
        let mut is_neighborhood_empty = true;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let (y, x) = (y as isize + i, x as isize + j);
                assert!(y >= 0 && x >= 0);
                let (y, x) = (y as usize, x as usize);
                assert!(y < self.tiles.len() && x < self.tiles[0].len());
                if self.tiles[y][x] != Tile::Open {
                    is_neighborhood_empty = false;
                }
            }
        }
        if is_neighborhood_empty {
            return None;
        }
        for i in 0..4 {
            let offsets = DIRECTIONS[(i + round) % 4];
            if offsets.iter().all(|(dy, dx)| {
                let (ny, nx) = (y as isize + dy, x as isize + dx);
                if ny < 0 || ny >= self.tiles.len() as isize {
                    return false;
                }
                if nx < 0 || nx >= self.tiles[ny as usize].len() as isize {
                    return false;
                }
                match self.tiles[ny as usize][nx as usize] {
                    Tile::Open => true,
                    Tile::Elf => false,
                }
            }) {
                return Some((
                    (y as isize + offsets[1].0) as usize,
                    (x as isize + offsets[1].1) as usize,
                ));
            }
        }
        None
    }

    fn relocate(&mut self, proposals: &[(usize, usize)]) {
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[y].len() {
                if proposals.contains(&(y, x)) {
                    self.tiles[y][x] = Tile::Elf;
                } else {
                    self.tiles[y][x] = Tile::Open;
                }
            }
        }
    }

    fn encompassing_rectangle(&self) -> ((usize, usize), (usize, usize)) {
        let mut min_y = self.tiles.len();
        let mut max_y = 0;
        let mut min_x = self.tiles[0].len();
        let mut max_x = 0;
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[y].len() {
                if self.tiles[y][x] == Tile::Elf {
                    min_y = min_y.min(y);
                    max_y = max_y.max(y);
                    min_x = min_x.min(x);
                    max_x = max_x.max(x);
                }
            }
        }
        ((min_y, min_x), (max_y, max_x))
    }

    fn count_elves_in_rectangle(
        &self,
        ((min_y, min_x), (max_y, max_x)): ((usize, usize), (usize, usize)),
    ) -> usize {
        let mut count = 0;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.tiles[y][x] == Tile::Elf {
                    count += 1;
                }
            }
        }
        count
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in &self.tiles {
            for tile in row {
                let c = match tile {
                    Tile::Open => '.',
                    Tile::Elf => '#',
                };
                print!("{c}");
            }
            println!();
        }
    }
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let lines = input.lines().collect::<Vec<_>>();
    let mut board = Board::new(&lines);
    for i in 0..10 {
        let mut proposals = board.propose(i).unwrap();
        proposals.sort_unstable();
        let mut next_pos = Vec::new();
        let mut j = 0;
        while j < proposals.len() {
            if j < proposals.len() - 1 && proposals[j].0 == proposals[j + 1].0 {
                while j < proposals.len() - 1 && proposals[j].0 == proposals[j + 1].0 {
                    next_pos.push(proposals[j].1);
                    j += 1;
                }
                next_pos.push(proposals[j].1);
            } else {
                next_pos.push(proposals[j].0);
            }
            j += 1;
        }
        board.relocate(&next_pos);
    }
    let ((min_y, min_x), (max_y, max_x)) = board.encompassing_rectangle();
    let count = board.count_elves_in_rectangle(((min_y, min_x), (max_y, max_x)));
    println!(
        "Part 1: {}",
        (max_x - min_x + 1) * (max_y - min_y + 1) - count
    );

    let mut board = Board::new(&lines);
    let mut i = 0;
    loop {
        let Some(mut proposals) = board.propose(i) else {
            break;
        };
        proposals.sort_unstable();
        let mut next_pos = Vec::new();
        let mut j = 0;
        while j < proposals.len() {
            if j < proposals.len() - 1 && proposals[j].0 == proposals[j + 1].0 {
                while j < proposals.len() - 1 && proposals[j].0 == proposals[j + 1].0 {
                    next_pos.push(proposals[j].1);
                    j += 1;
                }
                next_pos.push(proposals[j].1);
            } else {
                next_pos.push(proposals[j].0);
            }
            j += 1;
        }
        board.relocate(&next_pos);
        i += 1;
    }
    println!("Part 2: {}", i + 1);

    Ok(())
}
