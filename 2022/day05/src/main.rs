use std::io::{self, BufRead};

use anyhow::Result;

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut lines = io::BufReader::new(stdin).lines();

    let mut stacks = Vec::new();
    for line in lines.by_ref() {
        let line = line?;
        if line.starts_with(" 1") {
            break;
        }

        let stack_count = (line.len() + 1) / 4;
        while stack_count > stacks.len() {
            stacks.push(Vec::new());
        }

        for (i, cur) in (1..line.len()).step_by(4).enumerate() {
            let sym = line.as_bytes()[cur] as char;
            if sym == ' ' {
                continue;
            }
            stacks[i].push(sym);
        }
    }
    lines.next();

    for stack in &mut stacks {
        stack.reverse();
    }
    let mut stacks2 = stacks.clone();

    for line in lines {
        let line = line?;
        let mut iter = line.split(' ');
        iter.next();
        let num = iter
            .next()
            .ok_or_else(|| anyhow::anyhow!("invalid input"))?
            .parse::<usize>()?;
        iter.next();
        let from = iter
            .next()
            .ok_or_else(|| anyhow::anyhow!("invalid input"))?
            .parse::<usize>()?
            - 1;
        iter.next();
        let to = iter
            .next()
            .ok_or_else(|| anyhow::anyhow!("invalid input"))?
            .parse::<usize>()?
            - 1;

        let mut count = num;
        while count > 0 {
            let sym = stacks[from]
                .pop()
                .ok_or_else(|| anyhow::anyhow!("invalid input"))?;
            stacks[to].push(sym);
            count -= 1;
        }

        let to_move = stacks2[from][stacks2[from].len() - num..].to_vec();
        stacks2[to].extend(to_move);
        let new_len = stacks2[from].len() - num;
        stacks2[from].truncate(new_len);
    }

    print!("Part 1: ");
    for stack in &stacks {
        let sym = stack.last().copied().unwrap_or(' ');
        print!("{}", sym as char);
    }
    println!();
    print!("Part 2: ");
    for stack2 in &stacks2 {
        let sym = stack2.last().copied().unwrap_or(' ');
        print!("{}", sym as char);
    }
    println!();
    Ok(())
}
