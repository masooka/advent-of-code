use std::{fmt, io};

use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    None,
    Open,
    Solid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }

    fn facing(self) -> usize {
        match self {
            Self::Up => 3,
            Self::Down => 1,
            Self::Left => 2,
            Self::Right => 0,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Up => write!(f, "^"),
            Self::Down => write!(f, "v"),
            Self::Left => write!(f, "<"),
            Self::Right => write!(f, ">"),
        }
    }
}

#[derive(Clone)]
struct Board {
    tiles: Vec<Vec<Tile>>,
    pos: (usize, usize),
    dir: Direction,
}

impl Board {
    fn new(rows: &[&str]) -> Self {
        let mut tiles = rows
            .iter()
            .map(|row| {
                row.chars()
                    .map(|c| match c {
                        '.' => Tile::Open,
                        '#' => Tile::Solid,
                        _ => Tile::None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let max_col = tiles.iter().map(Vec::len).max().unwrap();
        for row in &mut tiles {
            row.resize(max_col, Tile::None);
        }
        let first_open = tiles[0].iter().position(|t| *t == Tile::Open).unwrap();
        Self {
            tiles,
            pos: (first_open, 0),
            dir: Direction::Right,
        }
    }

    fn action(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::TurnLeft => self.dir = self.dir.turn_left(),
            Instruction::TurnRight => self.dir = self.dir.turn_right(),
            Instruction::Move(n) => {
                for _ in 0..n {
                    let new_pos = self.step();
                    if self.tiles[new_pos.1][new_pos.0] == Tile::Open {
                        self.pos = new_pos;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn step(&self) -> (usize, usize) {
        let mut next_pos = self.pos;
        match self.dir {
            Direction::Up => loop {
                let new_y = if next_pos.1 == 0 {
                    self.tiles.len() - 1
                } else {
                    next_pos.1 - 1
                };
                if self.tiles[new_y][next_pos.0] != Tile::None {
                    next_pos.1 = new_y;
                    break;
                }
                next_pos.1 = new_y;
            },
            Direction::Down => loop {
                let new_y = if next_pos.1 == self.tiles.len() - 1 {
                    0
                } else {
                    next_pos.1 + 1
                };
                if self.tiles[new_y][next_pos.0] != Tile::None {
                    next_pos.1 = new_y;
                    break;
                }
                next_pos.1 = new_y;
            },
            Direction::Left => loop {
                let new_x = if next_pos.0 == 0 {
                    self.tiles[0].len() - 1
                } else {
                    next_pos.0 - 1
                };
                if self.tiles[next_pos.1][new_x] != Tile::None {
                    next_pos.0 = new_x;
                    break;
                }
                next_pos.0 = new_x;
            },
            Direction::Right => loop {
                let new_x = if next_pos.0 == self.tiles[0].len() - 1 {
                    0
                } else {
                    next_pos.0 + 1
                };
                if self.tiles[next_pos.1][new_x] != Tile::None {
                    next_pos.0 = new_x;
                    break;
                }
                next_pos.0 = new_x;
            },
        }
        next_pos
    }

    fn cube_action(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::TurnLeft => self.dir = self.dir.turn_left(),
            Instruction::TurnRight => self.dir = self.dir.turn_right(),
            Instruction::Move(n) => {
                for _ in 0..n {
                    let (new_pos, new_dir) = self.cube_step();
                    if self.tiles[new_pos.1][new_pos.0] == Tile::Open {
                        self.pos = new_pos;
                        self.dir = new_dir;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn cube_step(&self) -> ((usize, usize), Direction) {
        let mut next_pos = self.pos;
        let mut next_dir = self.dir;
        match self.dir {
            Direction::Up => {
                if next_pos.1 == 0 && 50 <= next_pos.0 && next_pos.0 < 100 {
                    next_pos = (0, next_pos.0 + 100);
                    next_dir = Direction::Right;
                } else if next_pos.1 == 0 && 100 <= next_pos.0 && next_pos.0 < 150 {
                    next_pos = (next_pos.0 - 100, 199);
                } else if next_pos.1 == 100 && next_pos.0 < 50 {
                    next_pos = (50, next_pos.0 + 50);
                    next_dir = Direction::Right;
                } else {
                    next_pos.1 -= 1;
                };
                assert!(next_pos.0 < 150);
                assert!(next_pos.1 < 200);
            }
            Direction::Down => {
                if next_pos.1 == 49 && 100 <= next_pos.0 && next_pos.0 < 150 {
                    next_pos = (99, next_pos.0 - 50);
                    next_dir = Direction::Left;
                } else if next_pos.1 == 149 && 50 <= next_pos.0 && next_pos.0 < 100 {
                    next_pos = (49, next_pos.0 + 100);
                    next_dir = Direction::Left;
                } else if next_pos.1 == 199 && next_pos.0 < 50 {
                    next_pos = (next_pos.0 + 100, 0);
                } else {
                    next_pos.1 += 1;
                };
                assert!(next_pos.0 < 150);
                assert!(next_pos.1 < 200);
            }
            Direction::Left => {
                if next_pos.0 == 0 && 100 <= next_pos.1 && next_pos.1 < 150 {
                    next_pos = (50, 149 - next_pos.1);
                    next_dir = Direction::Right;
                } else if next_pos.0 == 0 && 150 <= next_pos.1 && next_pos.1 < 200 {
                    next_pos = (next_pos.1 - 100, 0);
                    next_dir = Direction::Down;
                } else if next_pos.0 == 50 && next_pos.1 < 50 {
                    next_pos = (0, 149 - next_pos.1);
                    next_dir = Direction::Right;
                } else if next_pos.0 == 50 && 50 <= next_pos.1 && next_pos.1 < 100 {
                    next_pos = (next_pos.1 - 50, 100);
                    next_dir = Direction::Down;
                } else {
                    next_pos.0 -= 1;
                };
                assert!(next_pos.0 < 150);
                assert!(next_pos.1 < 200);
            }
            Direction::Right => {
                if next_pos.0 == 149 && next_pos.1 < 50 {
                    next_pos = (99, 149 - next_pos.1);
                    next_dir = Direction::Left;
                } else if next_pos.0 == 99 && 50 <= next_pos.1 && next_pos.1 < 100 {
                    next_pos = (next_pos.1 + 50, 49);
                    next_dir = Direction::Up;
                } else if next_pos.0 == 99 && 100 <= next_pos.1 && next_pos.1 < 150 {
                    next_pos = (149, 149 - next_pos.1);
                    next_dir = Direction::Left;
                } else if next_pos.0 == 49 && 150 <= next_pos.1 && next_pos.1 < 200 {
                    next_pos = (next_pos.1 - 100, 149);
                    next_dir = Direction::Up;
                } else {
                    next_pos.0 += 1;
                };
                assert!(next_pos.0 < 150);
                assert!(next_pos.1 < 200);
            }
        }
        (next_pos, next_dir)
    }

    #[allow(dead_code)]
    fn print(&self) {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if (x, y) == self.pos {
                    print!("{}", self.dir);
                } else {
                    match tile {
                        Tile::None => print!(" "),
                        Tile::Open => print!("."),
                        Tile::Solid => print!("#"),
                    }
                }
            }
            println!();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    Move(usize),
    TurnLeft,
    TurnRight,
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut num = String::new();
    for c in s.chars() {
        match c {
            '0'..='9' => num.push(c),
            'L' => {
                if !num.is_empty() {
                    instructions.push(Instruction::Move(num.parse().unwrap()));
                    num.clear();
                }
                instructions.push(Instruction::TurnLeft);
            }
            'R' => {
                if !num.is_empty() {
                    instructions.push(Instruction::Move(num.parse().unwrap()));
                    num.clear();
                }
                instructions.push(Instruction::TurnRight);
            }
            _ => {}
        }
    }
    if !num.is_empty() {
        instructions.push(Instruction::Move(num.parse().unwrap()));
    }
    instructions
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let lines = input.lines().collect::<Vec<_>>();
    let empty_line = lines.iter().position(|&l| l.is_empty()).unwrap();
    let (board, instructions) = (&lines[..empty_line], &lines[empty_line + 1..]);
    let mut board = Board::new(board);
    let mut cube_board = board.clone();
    let instructions = parse_instructions(instructions[0]);

    for &instr in &instructions {
        board.action(instr);
    }
    println!(
        "Part 1: {}",
        (board.pos.1 + 1) * 1000 + (board.pos.0 + 1) * 4 + board.dir.facing()
    );

    for &instr in &instructions {
        cube_board.cube_action(instr);
    }
    println!(
        "Part 2: {}",
        (cube_board.pos.1 + 1) * 1000 + (cube_board.pos.0 + 1) * 4 + cube_board.dir.facing()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cube_step() {
        let mut board = Board {
            pos: (0, 0),
            dir: Direction::Up,
            tiles: vec![vec![Tile::Solid; 200]; 150],
        };

        board.pos = (50, 0);
        board.dir = Direction::Up;
        assert_eq!(board.cube_step(), ((0, 150), Direction::Right));
        board.pos = (99, 0);
        board.dir = Direction::Up;
        assert_eq!(board.cube_step(), ((0, 199), Direction::Right));

        board.pos = (100, 0);
        board.dir = Direction::Up;
        assert_eq!(board.cube_step(), ((0, 199), Direction::Up));
        board.pos = (149, 0);
        board.dir = Direction::Up;
        assert_eq!(board.cube_step(), ((49, 199), Direction::Up));

        board.pos = (0, 100);
        board.dir = Direction::Up;
        assert_eq!(board.cube_step(), ((50, 50), Direction::Right));

        board.pos = (0, 199);
        board.dir = Direction::Down;
        assert_eq!(board.cube_step(), ((100, 0), Direction::Down));

        board.pos = (50, 149);
        board.dir = Direction::Down;
        assert_eq!(board.cube_step(), ((49, 150), Direction::Left));

        board.pos = (100, 49);
        board.dir = Direction::Down;
        assert_eq!(board.cube_step(), ((99, 50), Direction::Left));

        board.pos = (50, 0);
        board.dir = Direction::Left;
        assert_eq!(board.cube_step(), ((0, 149), Direction::Right));

        board.pos = (50, 50);
        board.dir = Direction::Left;
        assert_eq!(board.cube_step(), ((0, 100), Direction::Down));

        board.pos = (0, 100);
        board.dir = Direction::Left;
        assert_eq!(board.cube_step(), ((50, 49), Direction::Right));

        board.pos = (0, 150);
        board.dir = Direction::Left;
        assert_eq!(board.cube_step(), ((50, 0), Direction::Down));

        board.pos = (149, 0);
        board.dir = Direction::Right;
        assert_eq!(board.cube_step(), ((99, 149), Direction::Left));

        board.pos = (99, 50);
        board.dir = Direction::Right;
        assert_eq!(board.cube_step(), ((100, 49), Direction::Up));

        board.pos = (99, 100);
        board.dir = Direction::Right;
        assert_eq!(board.cube_step(), ((149, 49), Direction::Left));

        board.pos = (49, 150);
        board.dir = Direction::Right;
        assert_eq!(board.cube_step(), ((50, 149), Direction::Up));
    }
}
