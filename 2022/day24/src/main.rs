use std::io;

use anyhow::Result;

const MAX_ROUND: usize = 1500;

#[derive(Clone)]
struct Board {
    height: isize,
    width: isize,
    stream_up: Vec<Vec<bool>>,
    stream_down: Vec<Vec<bool>>,
    stream_left: Vec<Vec<bool>>,
    stream_right: Vec<Vec<bool>>,
    visited: Vec<Vec<Vec<bool>>>,
    shortest: usize,
}

impl Board {
    fn new(rows: &[&str]) -> Self {
        let height = rows.len() as isize - 2;
        let width = rows[0].len() as isize - 2;

        let mut stream_up = vec![vec![false; width as usize]; height as usize];
        let mut stream_down = vec![vec![false; width as usize]; height as usize];
        let mut stream_left = vec![vec![false; width as usize]; height as usize];
        let mut stream_right = vec![vec![false; width as usize]; height as usize];
        for (y, row) in rows[1..rows.len() - 1].iter().enumerate() {
            for (x, b) in row.as_bytes()[1..row.len() - 1].iter().enumerate() {
                match *b {
                    b'.' => continue,
                    b'^' => stream_up[y][x] = true,
                    b'v' => stream_down[y][x] = true,
                    b'<' => stream_left[y][x] = true,
                    b'>' => stream_right[y][x] = true,
                    _ => panic!("invalid character"),
                }
            }
        }

        Self {
            height,
            width,
            stream_up,
            stream_down,
            stream_left,
            stream_right,
            visited: vec![vec![vec![false; width as usize]; height as usize]; MAX_ROUND],
            shortest: usize::MAX,
        }
    }

    #[allow(clippy::cast_sign_loss)]
    fn step(&mut self, (y, x): (isize, isize), round: usize) {
        if round == MAX_ROUND {
            return;
        }
        if y >= 0 && self.visited[round][y as usize][x as usize] {
            return;
        }
        //self.path.push((y, x));
        if y == self.height - 1 && x == self.width - 1 {
            self.shortest = self.shortest.min(round + 1);
            return;
        }

        if y >= 0 && x < self.width - 1 && self.is_open((y, x + 1), round + 1) {
            self.step((y, x + 1), round + 1);
        }
        if y < self.height - 1 && self.is_open((y + 1, x), round + 1) {
            self.step((y + 1, x), round + 1);
        }
        if y >= 0 && self.is_open((y, x), round + 1) {
            self.step((y, x), round + 1);
        }
        if y > 0 && self.is_open((y - 1, x), round + 1) {
            self.step((y - 1, x), round + 1);
        }
        if y >= 0 && x > 0 && self.is_open((y, x - 1), round + 1) {
            self.step((y, x - 1), round + 1);
        }
        if y == 0 && x == 0 {
            self.step((y - 1, x), round + 1);
        }
        if y < 0 {
            self.step((y, x), round + 1);
        }

        if y >= 0 {
            self.visited[round][y as usize][x as usize] = true;
        }
        //self.path.pop();
    }

    fn is_open(&self, (y, x): (isize, isize), round: usize) -> bool {
        //println!("  {},{} at {}", y, x, round);
        !self.stream_up[(y + round as isize).rem_euclid(self.height) as usize][x as usize]
            && !self.stream_down[(y - round as isize).rem_euclid(self.height) as usize][x as usize]
            && !self.stream_left[y as usize][(x + round as isize).rem_euclid(self.width) as usize]
            && !self.stream_right[y as usize][(x - round as isize).rem_euclid(self.width) as usize]
    }

    #[allow(dead_code)]
    fn print(&self, (pos_y, pos_x): (isize, isize), round: usize) {
        for y in 0..self.height {
            for x in 0..self.width {
                let mut cnt_up = 0;
                let mut cnt_down = 0;
                let mut cnt_left = 0;
                let mut cnt_right = 0;

                if y == pos_y && x == pos_x {
                    print!("E");
                    continue;
                }

                if self.stream_up[(y + round as isize).rem_euclid(self.height) as usize][x as usize]
                {
                    cnt_up += 1;
                }
                if self.stream_down[(y - round as isize).rem_euclid(self.height) as usize]
                    [x as usize]
                {
                    cnt_down += 1;
                }
                if self.stream_left[y as usize]
                    [(x + round as isize).rem_euclid(self.width) as usize]
                {
                    cnt_left += 1;
                }
                if self.stream_right[y as usize]
                    [(x - round as isize).rem_euclid(self.width) as usize]
                {
                    cnt_right += 1;
                }

                let c = match (cnt_up, cnt_down, cnt_left, cnt_right) {
                    (0, 0, 0, 0) => b'.',
                    (1, 0, 0, 0) => b'^',
                    (0, 1, 0, 0) => b'v',
                    (0, 0, 1, 0) => b'<',
                    (0, 0, 0, 1) => b'>',
                    _ => b'0' + cnt_up + cnt_down + cnt_left + cnt_right,
                };
                print!("{}", c as char);
            }
            println!();
        }
    }
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let lines = input.lines().collect::<Vec<_>>();
    let mut board = Board::new(&lines);
    board.step((-1, 0), 0);
    println!("Part 1: {}", board.shortest);

    let start_round = board.shortest;
    let mut board_forward = Board::new(&lines);
    // Flip input
    let flipped = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '^' => 'v',
                    'v' => '^',
                    '<' => '>',
                    '>' => '<',
                    _ => c,
                })
                .rev()
                .collect::<String>()
        })
        .rev()
        .collect::<Vec<_>>();
    let lines = flipped.iter().map(String::as_str).collect::<Vec<_>>();
    let mut board_backward = Board::new(&lines);
    board_backward.step((-1, 0), start_round);
    board_forward.step((-1, 0), board_backward.shortest);
    println!("Part 2: {}", board_forward.shortest);

    Ok(())
}
