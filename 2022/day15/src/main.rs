use std::io;

use anyhow::{anyhow, Result};
use regex::Regex;

#[derive(Debug)]
struct Sensor {
    sensor_coord: (isize, isize),
    beacon_coord: (isize, isize),
}

impl Sensor {
    fn available(&self, coord: (isize, isize)) -> bool {
        self.distance(self.beacon_coord) < self.distance(coord)
    }

    fn unavailable_interval(&self, y: isize) -> Option<(isize, isize)> {
        let max_distance = self.distance(self.beacon_coord);
        let vertical_distance = (self.sensor_coord.1 - y).abs();
        let remaining = max_distance - vertical_distance;
        if remaining < 0 {
            return None;
        }
        Some((
            self.sensor_coord.0 - remaining,
            self.sensor_coord.0 + remaining,
        ))
    }

    fn distance(&self, coord: (isize, isize)) -> isize {
        (self.sensor_coord.0 - coord.0).abs() + (self.sensor_coord.1 - coord.1).abs()
    }
}

fn parse_sensor(re: &Regex, input: &str) -> Result<Sensor> {
    let caps = re
        .captures(input)
        .ok_or_else(|| anyhow!("invalid input: {}", input))?;
    let sensor_coord = (
        caps.get(1).unwrap().as_str().parse()?,
        caps.get(2).unwrap().as_str().parse()?,
    );
    let beacon_coord = (
        caps.get(3).unwrap().as_str().parse()?,
        caps.get(4).unwrap().as_str().parse()?,
    );
    Ok(Sensor {
        sensor_coord,
        beacon_coord,
    })
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let lines = input.lines().collect::<Vec<_>>();

    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .expect("valid regex");
    let sensors = lines
        .iter()
        .map(|line| parse_sensor(&re, line).unwrap())
        .collect::<Vec<_>>();

    // Finds the minimum and maximum x coordinates.
    let (min_x, max_x) = sensors
        .iter()
        .fold((isize::MAX, isize::MIN), |acc, sensor| {
            (
                acc.0.min(sensor.sensor_coord.0).min(sensor.beacon_coord.0),
                acc.1.max(sensor.sensor_coord.0).max(sensor.beacon_coord.0),
            )
        });

    let row = 2_000_000;
    let lower_margin = sensors
        .iter()
        .map(|sensor| sensor.distance((min_x, row)))
        .max()
        .unwrap();
    let upper_margin = sensors
        .iter()
        .map(|sensor| sensor.distance((max_x, row)))
        .max()
        .unwrap();
    let mut unavailable = 0;
    for x in min_x - lower_margin..=max_x + upper_margin {
        if sensors
            .iter()
            .any(|sensor| sensor.sensor_coord == (x, row) || sensor.beacon_coord == (x, row))
        {
            continue;
        }
        if sensors.iter().any(|sensor| !sensor.available((x, row))) {
            unavailable += 1;
        }
    }
    println!("Part 1: {}", unavailable);

    let mut distress_coord = (0, 0);
    for y in 0..=4_000_000 {
        let mut unavailable_intervals = sensors
            .iter()
            .filter_map(|sensor| sensor.unavailable_interval(y))
            .collect::<Vec<_>>();
        let max_x = unavailable_intervals
            .iter()
            .fold(isize::MIN, |acc, interval| acc.max(interval.1));
        unavailable_intervals.sort_by(|a, b| a.0.cmp(&b.0));
        let mut x = unavailable_intervals[0].1;
        while x < max_x {
            if let Some(interval) = unavailable_intervals
                .iter()
                .find(|interval| interval.0 <= x && x <= interval.1)
            {
                x = interval.1 + 1;
                continue;
            }
            distress_coord = (x, y);
            break;
        }
        if distress_coord != (0, 0) {
            break;
        }
    }
    assert!(sensors
        .iter()
        .all(|sensor| sensor.available(distress_coord)));
    println!(
        "Part 2: {}",
        distress_coord.0 * 4_000_000 + distress_coord.1
    );

    Ok(())
}
