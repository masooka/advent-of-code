#![allow(clippy::cast_sign_loss)]
use std::io;

use anyhow::Result;

#[derive(Debug)]
struct Text {
    text: Vec<isize>,
    forward: Vec<usize>,
    reverse: Vec<usize>,
}

impl Text {
    fn from_vec(text: Vec<isize>) -> Self {
        let forward = (0..text.len()).collect();
        let reverse = (0..text.len()).collect();
        Self {
            text,
            forward,
            reverse,
        }
    }

    fn text(&self) -> Vec<isize> {
        let mut decrypted = vec![0; self.text.len()];
        for (i, &c) in self.text.iter().enumerate() {
            decrypted[self.forward[i]] = c;
        }
        decrypted
    }

    fn coordinates(&self) -> isize {
        let text = self.text();
        let pos = text.iter().position(|&c| c == 0).unwrap();
        let a = text[(pos + 1000) % text.len()];
        let b = text[(pos + 2000) % text.len()];
        let c = text[(pos + 3000) % text.len()];
        a + b + c
    }

    fn move_left(&mut self, i: usize) {
        let pos = self.forward[i];
        if pos > 1 {
            self.forward.swap(i, self.reverse[pos - 1]);
            self.reverse.swap(pos, pos - 1);
        } else {
            let dst = self.reverse.len() - 2 + pos;
            for cur in pos..dst {
                self.forward.swap(self.reverse[cur], self.reverse[cur + 1]);
                self.reverse.swap(cur, cur + 1);
            }
        }
    }

    fn move_right(&mut self, i: usize) {
        let pos = self.forward[i];
        if pos < self.forward.len() - 1 {
            self.forward.swap(i, self.reverse[pos + 1]);
            self.reverse.swap(pos, pos + 1);
        } else {
            for cur in (2..self.reverse.len()).rev() {
                self.forward.swap(self.reverse[cur], self.reverse[cur - 1]);
                self.reverse.swap(cur, cur - 1);
            }
        }
    }

    fn mix(&mut self) {
        let original = self.text.clone();
        for (i, &x) in original.iter().enumerate() {
            let n = x.unsigned_abs() % (original.len() - 1);
            for _ in 0..n {
                if x > 0 {
                    self.move_right(i);
                } else {
                    self.move_left(i);
                }
            }
        }
    }
}

fn main() -> Result<()> {
    const DECRYPTION_KEY: isize = 811_589_153;

    let input = io::read_to_string(io::stdin())?;
    let lines = input.lines().collect::<Vec<_>>();
    let original = lines
        .iter()
        .map(|line| line.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    let mut text = Text::from_vec(original.clone());
    text.mix();
    println!("Part 1: {}", text.coordinates());

    let original = original
        .iter()
        .map(|&x| x * DECRYPTION_KEY)
        .collect::<Vec<_>>();
    let mut text = Text::from_vec(original);
    for _ in 0..10 {
        text.mix();
    }
    println!("Part 2: {}", text.coordinates());

    Ok(())
}
