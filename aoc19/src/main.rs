use std::{io, str::FromStr};

use anyhow::Result;

struct Cost {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

impl Cost {
    fn new(ore: usize, clay: usize, obsidian: usize) -> Self {
        Self {
            ore,
            clay,
            obsidian,
        }
    }
}

struct Blueprint {
    ore_robot_cost: Cost,
    clay_robot_cost: Cost,
    obsidian_robot_cost: Cost,
    geode_robot_cost: Cost,
}

impl Blueprint {
    fn simulate_dfs(&self, env: &Env, duration: usize, mut max_seen: usize) -> usize {
        if env.minutes == duration {
            return env.geode;
        }
        if self.geode_upper_bound(env, duration) < max_seen {
            return 0;
        }

        let mut new_env = Self::collect(env);
        new_env.minutes += 1;

        if self.can_build_geode_robot(env) {
            let new_env = self.build_geode_robot(&new_env);
            let tree_max = self.simulate_dfs(&new_env, duration, max_seen);
            if tree_max > max_seen {
                max_seen = tree_max;
            }
        }

        if self.can_build_obsidian_robot(env) {
            let new_env = self.build_obsidian_robot(&new_env);
            let tree_max = self.simulate_dfs(&new_env, duration, max_seen);
            if tree_max > max_seen {
                max_seen = tree_max;
            }
        }

        if self.can_build_clay_robot(env) {
            let new_env = self.build_clay_robot(&new_env);
            let tree_max = self.simulate_dfs(&new_env, duration, max_seen);
            if tree_max > max_seen {
                max_seen = tree_max;
            }
        }

        if self.can_build_ore_robot(env) {
            let new_env = self.build_ore_robot(&new_env);
            let tree_max = self.simulate_dfs(&new_env, duration, max_seen);
            if tree_max > max_seen {
                max_seen = tree_max;
            }
        }

        let tree_max = self.simulate_dfs(&new_env, duration, max_seen);
        if tree_max > max_seen {
            max_seen = tree_max;
        }

        max_seen
    }

    fn geode_upper_bound(&self, env: &Env, duration: usize) -> usize {
        let mut cur_env = env.clone();
        let mut new_geode_robots = cur_env.geode_robots;
        while cur_env.minutes < duration {
            if cur_env.obsidian >= self.geode_robot_cost.obsidian {
                cur_env.obsidian -= self.geode_robot_cost.obsidian;
                new_geode_robots += 1;
            }
            cur_env.geode += cur_env.geode_robots;
            cur_env.geode_robots = new_geode_robots;
            cur_env.obsidian += cur_env.obsidian_robots;
            cur_env.obsidian_robots += 1; // assumption
            cur_env.minutes += 1;
        }
        cur_env.geode
    }

    fn collect(env: &Env) -> Env {
        let mut env = env.clone();
        env.ore += env.ore_robots;
        env.clay += env.clay_robots;
        env.obsidian += env.obsidian_robots;
        env.geode += env.geode_robots;
        env
    }

    fn can_build_ore_robot(&self, env: &Env) -> bool {
        self.ore_robot_cost.ore <= env.ore
            && self.ore_robot_cost.clay <= env.clay
            && self.ore_robot_cost.obsidian <= env.obsidian
    }

    fn can_build_clay_robot(&self, env: &Env) -> bool {
        self.clay_robot_cost.ore <= env.ore
            && self.clay_robot_cost.clay <= env.clay
            && self.clay_robot_cost.obsidian <= env.obsidian
    }

    fn can_build_obsidian_robot(&self, env: &Env) -> bool {
        self.obsidian_robot_cost.ore <= env.ore
            && self.obsidian_robot_cost.clay <= env.clay
            && self.obsidian_robot_cost.obsidian <= env.obsidian
    }

    fn can_build_geode_robot(&self, env: &Env) -> bool {
        self.geode_robot_cost.ore <= env.ore
            && self.geode_robot_cost.clay <= env.clay
            && self.geode_robot_cost.obsidian <= env.obsidian
    }

    fn build_ore_robot(&self, env: &Env) -> Env {
        let mut env = env.clone();
        env.ore -= self.ore_robot_cost.ore;
        env.clay -= self.ore_robot_cost.clay;
        env.obsidian -= self.ore_robot_cost.obsidian;
        env.ore_robots += 1;
        env
    }

    fn build_clay_robot(&self, env: &Env) -> Env {
        let mut env = env.clone();
        env.ore -= self.clay_robot_cost.ore;
        env.clay -= self.clay_robot_cost.clay;
        env.obsidian -= self.clay_robot_cost.obsidian;
        env.clay_robots += 1;
        env
    }

    fn build_obsidian_robot(&self, env: &Env) -> Env {
        let mut env = env.clone();
        env.ore -= self.obsidian_robot_cost.ore;
        env.clay -= self.obsidian_robot_cost.clay;
        env.obsidian -= self.obsidian_robot_cost.obsidian;
        env.obsidian_robots += 1;
        env
    }

    fn build_geode_robot(&self, env: &Env) -> Env {
        let mut env = env.clone();
        env.ore -= self.geode_robot_cost.ore;
        env.clay -= self.geode_robot_cost.clay;
        env.obsidian -= self.geode_robot_cost.obsidian;
        env.geode_robots += 1;
        env
    }
}

impl FromStr for Blueprint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let tokens = s.split(' ').collect::<Vec<_>>();
        let ore_robot_cost = Cost::new(tokens[6].parse::<usize>()?, 0, 0);
        let clay_robot_cost = Cost::new(tokens[12].parse::<usize>()?, 0, 0);
        let obsidian_robot_cost = Cost::new(
            tokens[18].parse::<usize>()?,
            tokens[21].parse::<usize>()?,
            0,
        );
        let geode_robot_cost = Cost::new(
            tokens[27].parse::<usize>()?,
            0,
            tokens[30].parse::<usize>()?,
        );
        Ok(Self {
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        })
    }
}

#[derive(Clone, Debug)]
struct Env {
    minutes: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
}

impl Env {
    fn new() -> Self {
        Self {
            minutes: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let lines = input.lines().collect::<Vec<_>>();
    let blueprints = lines
        .iter()
        .map(|&s| s.parse::<Blueprint>().unwrap())
        .collect::<Vec<_>>();

    let init_env = Env::new();
    let mut total_quality_level = 0;
    for (i, blueprint) in blueprints.iter().enumerate() {
        let quality_level = blueprint.simulate_dfs(&init_env, 24, 0);
        total_quality_level += quality_level * (i + 1);
    }
    println!("Part 1: {total_quality_level}");

    let mut product = 1;
    for blueprint in blueprints.iter().take(3) {
        let quality_level = blueprint.simulate_dfs(&init_env, 32, 0);
        product *= quality_level;
    }
    println!("Part 2: {product}");

    Ok(())
}
