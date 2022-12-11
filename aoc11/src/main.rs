use std::{collections::VecDeque, io};

use anyhow::Result;

#[derive(Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operator: char,
    operand: usize,
    divisor: usize,
    target_t: usize,
    target_f: usize,
    count: usize,
}

impl Monkey {
    fn new(input: &[&str]) -> Self {
        let mut items = VecDeque::new();
        input[0][18..].split(',').for_each(|s| {
            items.push_back(s.trim().parse().unwrap());
        });

        let op_line = input[1][13..].split(' ').collect::<Vec<_>>();
        let (operator, operand) = match op_line.get(3) {
            Some(&"+") => {
                let operand = op_line[4].parse().unwrap();
                (op_line[3].chars().next().unwrap(), operand)
            }
            Some(&"*") => {
                if op_line[4] == "old" {
                    ('^', 2)
                } else {
                    let operand = op_line[4].parse().unwrap();
                    (op_line[3].chars().next().unwrap(), operand)
                }
            }
            _ => unreachable!(),
        };

        let divisor = input[2][21..].parse().unwrap();

        Self {
            items,
            operator,
            operand,
            divisor,
            target_t: input[3][29..].parse().unwrap(),
            target_f: input[4][30..].parse().unwrap(),
            count: 0,
        }
    }

    fn receive(&mut self, item: usize) {
        self.items.push_back(item);
    }
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let lines = input.split('\n').collect::<Vec<_>>();

    let mut monkeys = Vec::new();
    for monkey_input in lines.chunks(7) {
        let monkey = Monkey::new(&monkey_input[1..6]);
        monkeys.push(monkey);
    }

    let business_at_20 = business(monkeys.clone(), 20, 3);
    println!("Part 1: {}", business_at_20);
    let business_at_10000 = business(monkeys, 10000, 1);
    println!("Part 2: {}", business_at_10000);

    Ok(())
}

fn business(mut monkeys: Vec<Monkey>, rounds: usize, divisor: usize) -> usize {
    let common_multiplier: usize = monkeys.iter().map(|m| m.divisor).product();
    for _round in 1..=rounds {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                let worry_level = match &monkeys[i].operator {
                    '+' => (item + monkeys[i].operand) / divisor % common_multiplier,
                    '*' => (item * monkeys[i].operand) / divisor % common_multiplier,
                    '^' => (item * item) / divisor % common_multiplier,
                    _ => unreachable!(),
                };
                let target = if worry_level % monkeys[i].divisor == 0 {
                    monkeys[i].target_t
                } else {
                    monkeys[i].target_f
                };
                monkeys[i].count += 1;
                monkeys[target].receive(worry_level);
            }
        }
    }

    let mut count = monkeys.iter().map(|m| m.count).collect::<Vec<_>>();
    count.sort_unstable_by(|a, b| b.cmp(a));
    count[0] * count[1]
}
