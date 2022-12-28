use std::io;

use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RockKind {
    A, // -
    B, // +
    C, // _|
    D, // |
    E, // []
}

impl RockKind {
    #[allow(clippy::match_same_arms)]
    fn height(self) -> usize {
        match self {
            RockKind::A => 1,
            RockKind::B => 3,
            RockKind::C => 3,
            RockKind::D => 4,
            RockKind::E => 2,
        }
    }

    #[allow(clippy::match_same_arms)]
    fn width(self) -> usize {
        match self {
            RockKind::A => 4,
            RockKind::B => 3,
            RockKind::C => 3,
            RockKind::D => 1,
            RockKind::E => 2,
        }
    }

    fn iter() -> impl Iterator<Item = RockKind> {
        vec![
            RockKind::A,
            RockKind::B,
            RockKind::C,
            RockKind::D,
            RockKind::E,
        ]
        .into_iter()
        .cycle()
    }
}

#[derive(Clone, Debug)]
struct Rock {
    kind: RockKind,
    coord: (usize, usize),
}

impl Rock {
    fn new(kind: RockKind, coord: (usize, usize)) -> Self {
        Self { kind, coord }
    }

    fn x(&self) -> usize {
        self.coord.0
    }

    fn y(&self) -> usize {
        self.coord.1
    }

    fn max_y(&self) -> usize {
        self.y() + self.kind.height() - 1
    }

    fn iter(&self) -> impl Iterator<Item = (usize, usize)> {
        let mut coords = match self.kind {
            RockKind::A => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            RockKind::B => vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            RockKind::C => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            RockKind::D => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            RockKind::E => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        };
        for (x, y) in &mut coords {
            *x += self.x();
            *y += self.y();
        }
        coords.into_iter()
    }

    fn occupied(&self, coord: (usize, usize)) -> bool {
        let (x, y) = coord;
        if x < self.coord.0
            || x >= self.coord.0 + self.kind.width()
            || y < self.coord.1
            || y >= self.coord.1 + self.kind.height()
        {
            return false;
        }
        match self.kind {
            RockKind::A | RockKind::D | RockKind::E => true,
            RockKind::B => {
                coord == (self.coord.0, self.coord.1 + 1)
                    || coord == (self.coord.0 + 1, self.coord.1)
                    || coord == (self.coord.0 + 1, self.coord.1 + 1)
                    || coord == (self.coord.0 + 1, self.coord.1 + 2)
                    || coord == (self.coord.0 + 2, self.coord.1 + 1)
            }
            RockKind::C => {
                coord == (self.coord.0, self.coord.1)
                    || coord == (self.coord.0 + 1, self.coord.1)
                    || coord == (self.coord.0 + 2, self.coord.1)
                    || coord == (self.coord.0 + 2, self.coord.1 + 1)
                    || coord == (self.coord.0 + 2, self.coord.1 + 2)
            }
        }
    }

    fn move_left(&mut self) {
        self.coord.0 -= 1;
    }

    fn move_right(&mut self) {
        self.coord.0 += 1;
    }

    fn move_up(&mut self) {
        self.coord.1 += 1;
    }

    fn move_down(&mut self) {
        self.coord.1 -= 1;
    }
}

fn rock_collision(rock1: &Rock, rock2: &Rock) -> bool {
    match (rock1.kind, rock2.kind) {
        (RockKind::A, RockKind::A) => rock1.y() == rock2.y(),
        (RockKind::D, RockKind::D) => rock1.x() == rock2.x() && rock1.y().abs_diff(rock2.y()) < 4,
        (RockKind::E, RockKind::E) => {
            rock1.x().abs_diff(rock2.x()) < 2 && rock1.y().abs_diff(rock2.y()) < 2
        }
        _ => rock1.iter().any(|coord| rock2.occupied(coord)),
    }
}

enum JetDirection {
    Left,
    Right,
}

struct Jet {
    pattern: String,
}

impl Jet {
    fn new(pattern: &str) -> Self {
        Self {
            pattern: pattern.to_string(),
        }
    }

    fn iter(&self) -> impl Iterator<Item = JetDirection> + '_ {
        self.pattern
            .chars()
            .map(|c| match c {
                '<' => JetDirection::Left,
                '>' => JetDirection::Right,
                _ => panic!("invalid jet pattern"),
            })
            .cycle()
    }
}

struct Chamber {
    rocks: Vec<Rock>,
    heights: Vec<usize>,
    count: usize,
}

impl Chamber {
    fn new() -> Self {
        Self {
            rocks: Vec::new(),
            heights: Vec::new(),
            count: 0,
        }
    }

    fn add_rock(&mut self, kind: RockKind, jet: impl Iterator<Item = JetDirection>) {
        let mut falling_rock = Rock::new(kind, (3, self.height() + 4));

        'falling: for dir in jet {
            // being pushed by a jet
            match dir {
                JetDirection::Left => {
                    if falling_rock.x() > 1 {
                        falling_rock.move_left();
                        for (rock, height) in self.rocks.iter().zip(self.heights.iter()).rev() {
                            if *height < falling_rock.y() {
                                break;
                            }
                            if rock_collision(&falling_rock, rock) {
                                falling_rock.move_right();
                                break;
                            }
                        }
                    }
                }
                JetDirection::Right => {
                    if falling_rock.x() + kind.width() <= 7 {
                        falling_rock.move_right();
                        for (rock, height) in self.rocks.iter().zip(self.heights.iter()).rev() {
                            if *height < falling_rock.y() {
                                break;
                            }
                            if rock_collision(&falling_rock, rock) {
                                falling_rock.move_left();
                                break;
                            }
                        }
                    }
                }
            }

            // falling one unit down
            if falling_rock.y() == 1 {
                break;
            }
            falling_rock.move_down();
            for (rock, height) in self.rocks.iter().zip(self.heights.iter()).rev() {
                if *height < falling_rock.y() {
                    break;
                }
                if rock_collision(&falling_rock, rock) {
                    falling_rock.move_up();
                    break 'falling;
                }
            }
        }
        if self.heights.is_empty() {
            self.heights.push(falling_rock.max_y());
        } else {
            self.heights
                .push((*self.heights.iter().last().unwrap()).max(falling_rock.max_y()));
        }
        self.count += 1;
        self.rocks.push(falling_rock);
    }

    fn height(&self) -> usize {
        self.heights.iter().last().copied().unwrap_or(0)
    }

    fn find_repeat(&self, period: usize) -> usize {
        if self.rocks.len() < period {
            return 0;
        }
        let src = self.rocks.len() - 1;
        let mut dst = src;
        while dst > period {
            dst -= period;
            let diff = self.rocks[src].y() - self.rocks[dst].y();
            for i in 0..period {
                if self.rocks[src - i].x() != self.rocks[dst - i].x()
                    || self.rocks[src - i].y() - self.rocks[dst - i].y() != diff
                {
                    break;
                }
                if i == period - 1 {
                    return src - dst;
                }
            }
        }
        0
    }
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let period = if input.len() % 5 == 0 {
        input.len()
    } else {
        input.len() * 5
    };
    let jet = Jet::new(input.trim());
    let mut chamber = Chamber::new();
    assert_eq!(chamber.height(), 0);

    let mut jet_iter = jet.iter();
    for (i, kind) in RockKind::iter().enumerate() {
        if i == 2022 {
            break;
        }
        chamber.add_rock(kind, &mut jet_iter);
    }
    println!("Part 1: {}", chamber.height());

    let mut chamber = Chamber::new();
    let mut jet_iter = jet.iter();
    let mut final_height = 0;
    for (i, kind) in RockKind::iter().enumerate() {
        chamber.add_rock(kind, &mut jet_iter);
        if i % period == period - 1 {
            let repeat = chamber.find_repeat(period);
            if repeat > 0 {
                let repeat_height = chamber.heights[i] - chamber.heights[i - repeat];
                let remaining = 1_000_000_000_000 - i - 1;
                let cycles = remaining / repeat;
                let remainder = remaining % repeat;
                let remainder_height =
                    chamber.heights[i - repeat + remainder] - chamber.heights[i - repeat];
                final_height = chamber.heights[i] + cycles * repeat_height + remainder_height;
                break;
            }
        }
    }
    println!("Part 2: {final_height}");

    Ok(())
}
