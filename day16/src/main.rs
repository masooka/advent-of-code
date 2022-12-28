use std::{collections::HashMap, io, str::FromStr};

use anyhow::Result;
use itertools::iproduct;

struct ParsedValve {
    name: String,
    rate: usize,
    next: Vec<String>,
}

impl FromStr for ParsedValve {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts = s.split(' ').collect::<Vec<_>>();
        let name = parts[1].to_string();
        let rate = parts[4][5..parts[4].len() - 1].parse::<usize>()?;
        let next = parts[9..]
            .iter()
            .map(|&s| {
                if s.ends_with(',') {
                    s.strip_suffix(',').unwrap().to_string()
                } else {
                    s.to_string()
                }
            })
            .collect::<Vec<_>>();

        Ok(ParsedValve { name, rate, next })
    }
}

struct Valve {
    rate: usize,
    next: Vec<usize>,
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let lines = input.lines().collect::<Vec<_>>();
    let parsed_valves = lines
        .iter()
        .map(|&s| s.parse::<ParsedValve>().unwrap())
        .collect::<Vec<_>>();
    let valves = to_valves(&parsed_valves);
    let first_pos = parsed_valves.iter().position(|v| v.name == "AA").unwrap();
    let positive_rates = valves.iter().filter(|v| v.rate > 0).count();
    let total_rates = valves.iter().map(|v| v.rate).sum::<usize>();

    let pressure = explore(
        &valves,
        &mut Vec::new(),
        &mut Vec::new(),
        first_pos,
        0,
        positive_rates,
    );
    println!("Part 1: {pressure}");

    let pressure = double_explore(
        &valves,
        &mut Vec::new(),
        &mut Vec::new(),
        &mut Vec::new(),
        first_pos,
        first_pos,
        0,
        positive_rates,
        total_rates,
        0,
        0,
    );
    println!("Part 2: {}", pressure.0);

    Ok(())
}

fn to_valves(valves: &[ParsedValve]) -> Vec<Valve> {
    let mut valve_map = HashMap::new();
    for (i, valve) in valves.iter().enumerate() {
        valve_map.insert(valve.name.as_str(), i);
    }

    valves
        .iter()
        .map(|v| Valve {
            rate: v.rate,
            next: v.next.iter().map(|n| valve_map[n.as_str()]).collect(),
        })
        .collect()
}

fn explore(
    valves: &[Valve],
    open: &mut Vec<usize>,
    actions: &mut Vec<(usize, bool)>,
    cur: usize,
    mut minute: usize,
    positive_rates: usize,
) -> usize {
    minute += 1;
    if minute >= 30 {
        return 0;
    }
    if open.len() == positive_rates {
        return 0;
    }

    let cur_valve = &valves[cur];

    let mut max_pressure = 0;
    let mut max_unopened = 0;
    for (count, &(id, opened)) in actions.iter().rev().enumerate() {
        if opened {
            if count > 0 && id == cur {
                return 0;
            }
            break;
        }
        if id == cur {
            return 0;
        }
        if !open.contains(&id) && valves[id].rate > max_unopened {
            max_unopened = valves[id].rate;
        }
    }

    if cur_valve.rate > max_unopened && !open.contains(&cur) {
        // move after opening the valve
        open.push(cur);
        actions.push((cur, true));

        let pressure = explore(valves, open, actions, cur, minute, positive_rates);
        max_pressure = pressure + cur_valve.rate * (30 - minute);

        actions.pop();
        open.pop();
    }

    // move without opening the valve
    actions.push((cur, false));

    for &next in &cur_valve.next {
        let pressure = explore(valves, open, actions, next, minute, positive_rates);
        if pressure > max_pressure {
            max_pressure = pressure;
        }
    }

    actions.pop();
    max_pressure
}

#[allow(clippy::too_many_lines, clippy::too_many_arguments)]
fn double_explore(
    valves: &[Valve],
    open: &mut Vec<usize>,
    actions1: &mut Vec<(usize, bool)>,
    actions2: &mut Vec<(usize, bool)>,
    cur1: usize,
    cur2: usize,
    mut minute: usize,
    positive_rates: usize,
    remaining_rates: usize,
    cur_pressure: usize,
    mut max_pressure_seen: usize,
) -> (usize, usize) {
    minute += 1;
    if minute >= 26 {
        return (0, max_pressure_seen);
    }
    if open.len() == positive_rates {
        return (0, max_pressure_seen);
    }
    if cur_pressure + remaining_rates * (26 - minute) < max_pressure_seen {
        return (0, max_pressure_seen);
    }

    let cur_valve1 = &valves[cur1];
    let cur_valve2 = &valves[cur2];

    let mut max_pressure = 0;
    let mut max_unopened1 = 0;
    for (count, &(id, opened)) in actions1.iter().rev().enumerate() {
        if opened {
            if count > 0 && id == cur1 {
                return (0, max_pressure_seen);
            }
            break;
        }
        if id == cur1 {
            return (0, max_pressure_seen);
        }
        if !open.contains(&id) && valves[id].rate > max_unopened1 {
            max_unopened1 = valves[id].rate;
        }
    }
    let mut max_unopened2 = 0;
    let mut max_record = Vec::new();
    for (count, &(id, opened)) in actions2.iter().rev().enumerate() {
        if opened {
            if count > 0 && id == cur2 {
                return (0, max_pressure_seen);
            }
            break;
        }
        if id == cur2 {
            return (0, max_pressure_seen);
        }
        if !open.contains(&id) && valves[id].rate > max_unopened2 {
            max_unopened2 = valves[id].rate;
        }
    }

    let can_open1 = cur_valve1.rate > max_unopened1 && !open.contains(&cur1);
    let can_open2 = cur_valve2.rate > max_unopened2 && !open.contains(&cur2);

    if can_open1 && can_open2 && cur1 != cur2 {
        // both opens
        open.push(cur1);
        open.push(cur2);
        actions1.push((cur1, true));
        actions2.push((cur2, true));

        let additional_pressure = cur_valve1.rate * (26 - minute) + cur_valve2.rate * (26 - minute);
        max_pressure_seen = max_pressure_seen.max(cur_pressure + additional_pressure);
        let (pressure, new_max_pressure_seen) = double_explore(
            valves,
            open,
            actions1,
            actions2,
            cur1,
            cur2,
            minute,
            positive_rates,
            remaining_rates - cur_valve1.rate - cur_valve2.rate,
            cur_pressure + additional_pressure,
            max_pressure_seen,
        );
        max_pressure = pressure + additional_pressure;
        max_pressure_seen = new_max_pressure_seen;
        max_record.push((minute, cur1));
        max_record.push((minute, cur2));

        actions1.pop();
        actions2.pop();
        open.pop();
        open.pop();
    }

    if can_open1 {
        // first opens and second moves
        open.push(cur1);
        actions1.push((cur1, true));
        actions2.push((cur2, false));

        let mut sibling_max_pressure = 0;
        let additional_pressure = cur_valve1.rate * (26 - minute);
        for &next in &cur_valve2.next {
            max_pressure_seen = max_pressure_seen.max(cur_pressure + additional_pressure);
            let (pressure, new_max_pressure_seen) = double_explore(
                valves,
                open,
                actions1,
                actions2,
                cur1,
                next,
                minute,
                positive_rates,
                remaining_rates - cur_valve1.rate,
                cur_pressure + additional_pressure,
                max_pressure_seen,
            );
            if pressure > sibling_max_pressure {
                sibling_max_pressure = pressure;
                max_pressure_seen = new_max_pressure_seen;
            }
        }
        sibling_max_pressure += additional_pressure;
        if sibling_max_pressure > max_pressure {
            max_pressure = sibling_max_pressure;
            max_record.push((minute, cur1));
        }

        actions1.pop();
        actions2.pop();
        open.pop();
    }

    if can_open2 {
        // first moves and second opens
        open.push(cur2);
        actions1.push((cur1, false));
        actions2.push((cur2, true));

        let mut sibling_max_pressure = 0;
        let additional_pressure = cur_valve2.rate * (26 - minute);
        for &next in &cur_valve1.next {
            max_pressure_seen = max_pressure_seen.max(cur_pressure + additional_pressure);
            let (pressure, new_max_pressure_seen) = double_explore(
                valves,
                open,
                actions1,
                actions2,
                next,
                cur2,
                minute,
                positive_rates,
                remaining_rates - cur_valve2.rate,
                cur_pressure + additional_pressure,
                max_pressure_seen,
            );
            if pressure > sibling_max_pressure {
                sibling_max_pressure = pressure;
                max_pressure_seen = new_max_pressure_seen;
            }
        }
        sibling_max_pressure += additional_pressure;
        if sibling_max_pressure > max_pressure {
            max_pressure = sibling_max_pressure;
            max_record.push((minute, cur2));
        }

        actions1.pop();
        actions2.pop();
        open.pop();
    }

    // both move without opening any valve
    actions1.push((cur1, false));
    actions2.push((cur2, false));

    for (&next1, &next2) in iproduct!(cur_valve1.next.iter(), cur_valve2.next.iter()) {
        let (pressure, new_max_pressure_seen) = double_explore(
            valves,
            open,
            actions1,
            actions2,
            next1,
            next2,
            minute,
            positive_rates,
            remaining_rates,
            cur_pressure,
            max_pressure_seen,
        );
        max_pressure_seen = new_max_pressure_seen;
        if pressure > max_pressure {
            max_pressure = pressure;
        }
    }

    actions1.pop();
    actions2.pop();
    (max_pressure, max_pressure_seen)
}
