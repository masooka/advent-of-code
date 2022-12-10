use std::io;

use anyhow::Result;

struct Crt {
    pixels: Vec<bool>,
}

impl Crt {
    fn new() -> Self {
        Self {
            pixels: vec![false; 6 * 40],
        }
    }

    fn set(&mut self, cycle: usize) {
        self.pixels[(cycle - 1) % (40 * 6)] = true;
    }

    fn display(&self) {
        for y in 0..6 {
            for x in 0..40 {
                print!("{}", if self.pixels[y * 40 + x] { "#" } else { "." });
            }
            println!();
        }
    }
}

#[allow(clippy::cast_sign_loss)]
fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let mut lines = input.split('\n').collect::<Vec<_>>();
    lines.pop();

    let mut x = 1;
    let mut cycle = 0;
    let sampling = vec![20, 60, 100, 140, 180, 220];
    let mut sampling_iter = sampling.iter();
    let mut sampling_cycle = sampling_iter.next().unwrap();
    let mut sum = 0;
    let mut crt = Crt::new();
    for line in lines {
        let mut iter = line.split(' ');
        let instruction = iter.next().unwrap();
        let argument = iter.next().unwrap_or("0").parse::<i32>()?;

        match instruction {
            "addx" => {
                cycle += 1;
                if visible(cycle, x) {
                    crt.set(cycle);
                }
                cycle += 1;
                if visible(cycle, x) {
                    crt.set(cycle);
                }
                if cycle >= *sampling_cycle {
                    sum += x as usize * *sampling_cycle;
                    if let Some(next) = sampling_iter.next() {
                        sampling_cycle = next;
                    } else {
                        sampling_cycle = &usize::MAX;
                    }
                }
                x += argument;
            }
            "noop" => {
                cycle += 1;
                if visible(cycle, x) {
                    crt.set(cycle);
                }
                if cycle >= *sampling_cycle {
                    sum += x as usize * *sampling_cycle;
                    if let Some(next) = sampling_iter.next() {
                        sampling_cycle = next;
                    } else {
                        sampling_cycle = &usize::MAX;
                    }
                }
            }
            _ => panic!("Unknown instruction"),
        }
    }

    println!("Part 1: {}", sum);
    println!("Part 2:");
    crt.display();
    Ok(())
}

#[allow(clippy::cast_sign_loss)]
fn visible(cycle: usize, x: i32) -> bool {
    let pos = (cycle - 1) % 40;
    if x < 1 {
        pos <= x as usize + 1
    } else {
        x as usize - 1 <= pos && pos <= x as usize + 1
    }
}
