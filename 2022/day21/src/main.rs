use std::{collections::HashMap, io, str::FromStr};

use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl From<char> for Op {
    fn from(c: char) -> Self {
        match c {
            '+' => Op::Add,
            '-' => Op::Sub,
            '*' => Op::Mul,
            '/' => Op::Div,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Job {
    Number(isize),
    Operation((Op, String, String)),
}

#[derive(Debug)]
struct Monkey {
    name: String,
    job: Job,
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split_whitespace();
        let name = parts.next().unwrap().chars().take(4).collect();
        let operand = parts.next().unwrap();
        if let Some(operator) = parts.next() {
            let op = operator.chars().next().unwrap().into();
            let lhs = operand;
            let rhs = parts.next().unwrap();
            Ok(Monkey {
                name,
                job: Job::Operation((op, lhs.to_string(), rhs.to_string())),
            })
        } else {
            let number = operand.parse()?;
            Ok(Monkey {
                name,
                job: Job::Number(number),
            })
        }
    }
}

struct Monkeys {
    monkeys: HashMap<String, Job>,
}

impl Monkeys {
    fn yell(&self) -> isize {
        self.eval("root")
    }

    fn eval(&self, name: &str) -> isize {
        match self.monkeys.get(name).unwrap() {
            Job::Number(n) => *n,
            Job::Operation((op, lhs, rhs)) => {
                let lhs = self.eval(lhs);
                let rhs = self.eval(rhs);
                match op {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Mul => lhs * rhs,
                    Op::Div => lhs / rhs,
                }
            }
        }
    }

    fn cmp(&mut self, lhs: &str, rhs: &str, n: isize) -> std::cmp::Ordering {
        self.monkeys.insert("humn".to_string(), Job::Number(n));
        let l_num = self.eval(lhs);
        let r_num = self.eval(rhs);
        l_num.cmp(&r_num)
    }
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let lines = input.lines().collect::<Vec<_>>();
    let jobs = lines
        .iter()
        .map(|&s| s.parse::<Monkey>().unwrap())
        .map(|m| (m.name, m.job))
        .collect::<HashMap<_, _>>();
    let mut monkeys = Monkeys { monkeys: jobs };

    let answer = monkeys.yell();
    println!("Part 1: {answer}");

    let (lhs, rhs) = match monkeys.monkeys.get_mut("root").unwrap() {
        Job::Operation((_, lhs, rhs)) => (lhs.clone(), rhs.clone()),
        Job::Number(_) => unreachable!(),
    };
    let mut min = 0;
    let mut max = 10_000_000_000_000;
    let mut answer;

    loop {
        let middle = (min + max) / 2;
        match monkeys.cmp(&lhs, &rhs, middle) {
            std::cmp::Ordering::Less => max = middle + 1,
            std::cmp::Ordering::Greater => min = middle - 1,
            std::cmp::Ordering::Equal => {
                answer = middle;
                break;
            }
        }
    }
    while monkeys.cmp(&lhs, &rhs, answer) == std::cmp::Ordering::Equal {
        answer -= 1;
    }

    println!("Part 2: {}", answer + 1);

    Ok(())
}
