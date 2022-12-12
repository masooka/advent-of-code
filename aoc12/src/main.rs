use std::io;

use anyhow::Result;

#[derive(Clone)]
struct Map {
    data: Vec<Vec<u8>>,
    visited: Vec<Vec<usize>>,
    path: Vec<(usize, usize)>,
}

impl Map {
    fn height(&self, pos: (usize, usize)) -> u8 {
        self.data[pos.0][pos.1]
    }

    fn visit(&mut self, pos: (usize, usize)) {
        let cur = *self.path.last().unwrap();
        self.visited[cur.0][cur.1] = self.path.len();
        self.path.push(pos);
    }

    fn unvisit(&mut self) {
        self.path.pop().unwrap();
    }

    fn distance(&self, pos: (usize, usize)) -> usize {
        self.visited[pos.0][pos.1]
    }

    fn step(&mut self) -> Vec<(usize, usize)> {
        let mut next_pos = Vec::new();
        let cur = *self.path.last().unwrap();
        if cur.0 > 0 {
            let new_pos = (cur.0 - 1, cur.1);
            if self.is_valid(new_pos) {
                next_pos.push(new_pos);
            }
        }
        if cur.1 > 0 {
            let new_pos = (cur.0, cur.1 - 1);
            if self.is_valid(new_pos) {
                next_pos.push(new_pos);
            }
        }
        if cur.0 < self.data.len() - 1 {
            let new_pos = (cur.0 + 1, cur.1);
            if self.is_valid(new_pos) {
                next_pos.push(new_pos);
            }
        }
        if cur.1 < self.data[0].len() - 1 {
            let new_pos = (cur.0, cur.1 + 1);
            if self.is_valid(new_pos) {
                next_pos.push(new_pos);
            }
        }

        let mut shortest_path = Vec::new();
        for new_pos in next_pos {
            if self.height(new_pos) == b'E'
                && (self.height(cur) == b'z' || self.height(cur) == b'y')
            {
                if shortest_path.is_empty() || self.path.len() < shortest_path.len() {
                    shortest_path = self.path.clone();
                }
                return shortest_path;
            }
            self.visit(new_pos);
            let new_path = self.step();
            if !new_path.is_empty()
                && (shortest_path.is_empty() || new_path.len() < shortest_path.len())
            {
                shortest_path = new_path;
            }
            self.unvisit();
        }

        shortest_path
    }

    fn is_valid(&self, pos: (usize, usize)) -> bool {
        let cur = *self.path.last().unwrap();
        (self.distance(pos) > self.path.len() + 1)
            && ((self.height(pos) == b'E' && self.height(cur) == b'z')
                || self.height(pos) <= self.height(cur) + 1)
    }
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let mut lines = input.split('\n').collect::<Vec<_>>();
    lines.pop();

    let data = lines.iter().map(|line| line.as_bytes().to_vec()).collect();
    let mut map = Map {
        data,
        visited: vec![vec![usize::MAX; lines[0].len()]; lines.len()],
        path: vec![(0, 0)],
    };

    // Find 'S'
    let mut start = (0, 0);
    for (i, row) in map.data.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == b'S' {
                start = (i, j);
                break;
            }
        }
    }
    map.data[start.0][start.1] = b'a';

    let mut map = map.clone();
    map.path = vec![start];
    let shortest_path = map.step();
    println!("Part 1: {}", shortest_path.len());

    let mut shortest_distance = usize::MAX;
    for i in 0..map.data.len() {
        for j in 0..map.data[0].len() {
            if map.data[i][j] == b'a' {
                let mut map = map.clone();
                map.path = vec![(i, j)];
                let shortest_path = map.step();
                if !shortest_path.is_empty() && shortest_path.len() < shortest_distance {
                    shortest_distance = shortest_path.len();
                }
            }
        }
    }
    println!("Part 2: {}", shortest_distance);

    Ok(())
}
