use std::{collections::HashSet, io};

use anyhow::Result;

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let mut map = input.split('\n').collect::<Vec<_>>();
    map.pop();

    let mut pos = vec![(0, 0); 10];
    let mut visited1 = HashSet::new();
    visited1.insert(pos[1]);
    let mut visited9 = HashSet::new();
    visited9.insert(pos[9]);

    for motion in map {
        let mut tokens = motion.split(' ');
        let direction = tokens.next().unwrap();
        let distance = tokens.next().unwrap().parse::<usize>()?;

        let (dx, dy) = match direction {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => unreachable!(),
        };
        for _ in 0..distance {
            pos[0].0 += dx;
            pos[0].1 += dy;
            for i in 1..10 {
                pos[i] = move_tail(pos[i - 1], pos[i]);
            }
            visited1.insert(pos[1]);
            visited9.insert(pos[9]);
        }
    }
    println!("Part 1: {}", visited1.len());
    println!("Part 2: {}", visited9.len());
    Ok(())
}

fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    if head.0 == tail.0 - 2 {
        if head.1 == tail.1 - 2 {
            (tail.0 - 1, tail.1 - 1)
        } else if head.1 == tail.1 + 2 {
            (tail.0 - 1, tail.1 + 1)
        } else {
            (tail.0 - 1, head.1)
        }
    } else if head.0 == tail.0 + 2 {
        if head.1 == tail.1 - 2 {
            (tail.0 + 1, tail.1 - 1)
        } else if head.1 == tail.1 + 2 {
            (tail.0 + 1, tail.1 + 1)
        } else {
            (tail.0 + 1, head.1)
        }
    } else if head.1 == tail.1 - 2 {
        if head.0 == tail.0 - 2 {
            (tail.0 - 1, tail.1 - 1)
        } else if head.0 == tail.0 + 2 {
            (tail.0 + 1, tail.1 - 1)
        } else {
            (head.0, tail.1 - 1)
        }
    } else if head.1 == tail.1 + 2 {
        if head.0 == tail.0 - 2 {
            (tail.0 - 1, tail.1 + 1)
        } else if head.0 == tail.0 + 2 {
            (tail.0 + 1, tail.1 + 1)
        } else {
            (head.0, tail.1 + 1)
        }
    } else {
        tail
    }
}
