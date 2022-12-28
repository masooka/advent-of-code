use std::{io, str};

use anyhow::Result;

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let mut map = input.split('\n').map(str::as_bytes).collect::<Vec<_>>();
    map.pop();
    let height = map.len();
    let width = map[0].len();

    let mut visible = 0;
    for i in 0..height {
        for j in 0..width {
            let tree_height = map[i][j];

            if (0..i).all(|k| map[k][j] < tree_height) {
                visible += 1;
                continue;
            }
            if (i + 1..height).all(|k| map[k][j] < tree_height) {
                visible += 1;
                continue;
            }
            if (0..j).all(|k| map[i][k] < tree_height) {
                visible += 1;
                continue;
            }
            if (j + 1..width).all(|k| map[i][k] < tree_height) {
                visible += 1;
                continue;
            }
        }
    }

    println!("Part 1: {}", visible);

    let mut max_score = 0;
    for i in 0..height {
        for j in 0..width {
            let tree_height = map[i][j];
            let mut score = 1;

            let mut distance = 0;
            for row in map[..i].iter().rev() {
                if row[j] >= tree_height {
                    distance += 1;
                    break;
                }
                distance += 1;
            }
            score *= distance;

            let mut distance = 0;
            for row in map.iter().skip(i + 1) {
                if row[j] >= tree_height {
                    distance += 1;
                    break;
                }
                distance += 1;
            }
            score *= distance;

            let mut distance = 0;
            for cell in map[i][..j].iter().rev() {
                if *cell >= tree_height {
                    distance += 1;
                    break;
                }
                distance += 1;
            }
            score *= distance;

            let mut distance = 0;
            for cell in map[i].iter().skip(j + 1) {
                if *cell >= tree_height {
                    distance += 1;
                    break;
                }
                distance += 1;
            }
            score *= distance;

            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("Part 2: {}", max_score);

    Ok(())
}
