// Day 19
// Robots

use regex::Regex;
use std::io::{self, BufRead};
use std::ops::{Add, Sub};
use std::ops::{AddAssign, SubAssign};

#[derive(Debug, Clone, Copy)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

// https://doc.rust-lang.org/std/ops/trait.Add.html
impl Add for Resources {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }
}
impl Sub for Resources {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}
// https://doc.rust-lang.org/std/ops/trait.SubAssign.html
impl SubAssign for Resources {
    fn sub_assign(&mut self, other: Self) {
        self.ore -= other.ore;
        self.clay -= other.clay;
        self.obsidian -= other.obsidian;
        self.geode -= other.geode;
    }
}
impl AddAssign for Resources {
    fn add_assign(&mut self, other: Self) {
        self.ore += other.ore;
        self.clay += other.clay;
        self.obsidian += other.obsidian;
        self.geode += other.geode;
    }
}

fn get_max_geodes(
    time_limit: usize,
    ore_cost: Resources,
    clay_cost: Resources,
    obsidian_cost: Resources,
    geode_cost: Resources,
) -> usize {
    let minutes = 0;

    let available_resources = Resources {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };
    let resource_rate = Resources {
        ore: 1,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };

    let mut state_stack = Vec::new();
    state_stack.push((minutes, available_resources, resource_rate));
    let mut best_geode_count = 0;
    let mut states_explored = 0;
    let mut states_pruned = 0;

    // dfs on resource states
    while !state_stack.is_empty() {
        let state = state_stack.pop().unwrap();
        states_explored += 1;
        let (minutes, resources, rates) = state;

        if minutes > time_limit {
            continue;
        }
        let minutes_left = time_limit - minutes;
        let upper_limit_geodes_left =
            ((minutes_left + rates.geode) * (minutes_left - 1 + rates.geode)) / 2;
        if resources.geode + (upper_limit_geodes_left) < best_geode_count {
            states_pruned += 1;
            continue;
        }
        if resources.geode > best_geode_count {
            best_geode_count = resources.geode;
        }

        // ALWAYS pick to build a geode robot over anything else if we have the resources
        // we can't spend obsidian on anything else and this will always increase output
        // the problem purists say this falls apart if the geode costs do not require obsidian
        // but i don't care for making things more difficult unnecessarily
        if resources.ore >= geode_cost.ore && resources.obsidian >= geode_cost.obsidian {
            // build a geode robot
            let new_rate = Resources {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 1,
            };
            state_stack.push((
                minutes + 1,
                resources + rates - geode_cost,
                rates + new_rate,
            ))
        } else {
            // if we can't build a geode robot we should always build an obsidian
            // robot if we can as it should help us make more geode robots
            if resources.ore >= obsidian_cost.ore && resources.clay >= obsidian_cost.clay {
                // build obsidian robot
                let new_rate = Resources {
                    ore: 0,
                    clay: 0,
                    obsidian: 1,
                    geode: 0,
                };
                state_stack.push((
                    minutes + 1,
                    resources + rates - obsidian_cost,
                    rates + new_rate,
                ))
            } else {
                // if we can't build a geode or obsidian robot we should try and build
                // clay and ore robots as well as just try doing nothing to save up
                // we don't build clay and ore robots if we have twice their cost
                // laying around idle which culls the problem space considerably
                // that last rule is a bit of a guess but seems to work :shrug:

                // build clay robot (if we're not holding twice the cost)
                if resources.ore >= clay_cost.ore && resources.ore < (clay_cost.ore * 2) {
                    // build clay robot
                    let new_rate = Resources {
                        ore: 0,
                        clay: 1,
                        obsidian: 0,
                        geode: 0,
                    };
                    state_stack.push((minutes + 1, resources + rates - clay_cost, rates + new_rate))
                } else {
                    states_pruned += 1
                }

                // build ore robot (if we're not holding twice the cost)
                if resources.ore >= ore_cost.ore && resources.ore < (ore_cost.ore * 2) {
                    let new_rate = Resources {
                        ore: 1,
                        clay: 0,
                        obsidian: 0,
                        geode: 0,
                    };
                    state_stack.push((minutes + 1, resources + rates - ore_cost, rates + new_rate))
                } else {
                    states_pruned += 1
                }

                // do nothing
                // save up for something nice?
                state_stack.push((minutes + 1, resources + rates, rates))
            }
        }
    }
    println!("States explored: {}", states_explored);
    println!("States pruned: {}", states_pruned);
    println!("Geodes cracked: {}", best_geode_count);
    best_geode_count
}

fn main() {
    let capture_spec = Regex::new(r"Blueprint (?P<blueprint>\d+): Each ore robot costs (?P<ore_clay_cost>\d+) ore. Each clay robot costs (?P<clay_ore_cost>\d+) ore. Each obsidian robot costs (?P<obsidian_ore_cost>\d+) ore and (?P<obsidian_clay_cost>\d+) clay. Each geode robot costs (?P<geode_ore_cost>\d+) ore and (?P<geode_obsidian_cost>\d+) obsidian.").unwrap();

    let stdin = io::stdin();
    let mut blueprint_i = 1;
    let mut total_qscore = 0;
    let mut geode_factor = 1;
    for line in stdin.lock().lines() {
        let this_line = line.unwrap();
        let matches = capture_spec.captures(&this_line).unwrap();

        let ore_cost = Resources {
            ore: matches["ore_clay_cost"].parse().unwrap(),
            clay: 0,
            obsidian: 0,
            geode: 0,
        };
        let clay_cost = Resources {
            ore: matches["clay_ore_cost"].parse().unwrap(),
            clay: 0,
            obsidian: 0,
            geode: 0,
        };
        let obsidian_cost = Resources {
            ore: matches["obsidian_ore_cost"].parse().unwrap(),
            clay: matches["obsidian_clay_cost"].parse().unwrap(),
            obsidian: 0,
            geode: 0,
        };
        let geode_cost = Resources {
            ore: matches["geode_ore_cost"].parse().unwrap(),
            clay: 0,
            obsidian: matches["geode_obsidian_cost"].parse().unwrap(),
            geode: 0,
        };

        // part 1
        let max_geodes = get_max_geodes(24, ore_cost, clay_cost, obsidian_cost, geode_cost);
        let quality_level = blueprint_i * max_geodes;
        println!(
            "Blueprint {}: maxg={}, qscore={}",
            blueprint_i, max_geodes, quality_level
        );
        total_qscore += quality_level;

        // part 2
        if blueprint_i <= 3 {
            let max_geodes = get_max_geodes(32, ore_cost, clay_cost, obsidian_cost, geode_cost);
            geode_factor *= max_geodes;
        }
        blueprint_i += 1;
    }
    println!("***");
    println!("Quality level aggregate: {}", total_qscore);
    println!("Geode factor: {}", geode_factor);
}
