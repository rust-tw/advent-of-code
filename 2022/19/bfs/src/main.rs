use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashSet, VecDeque};

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
}

#[derive(Default, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct State {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
}

impl State {
    fn collect(mut self) -> State {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
        self
    }
}

fn max_geodes(
    (_id, ore_cost, clay_cost, obsidian_ore_cost, obsidian_clay_cost, geode_ore_cost, geode_obsidian_cost): (usize, usize, usize, usize, usize, usize, usize),
    minute: usize,
) -> usize {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    let mut state = State::default();
    state.ore_robots = 1;
    queue.push_back((minute, state));

    let mut max_geodes = 0;
    let max_ore_cost = [ore_cost, clay_cost, obsidian_ore_cost, geode_ore_cost]
        .into_iter()
        .max()
        .unwrap();
    while let Some((minute, mut state)) = queue.pop_front() {
        if minute == 0 {
            max_geodes = std::cmp::max(max_geodes, state.geode);
            continue;
        }

        state.ore = state.ore.min(
            max_ore_cost + (max_ore_cost - state.ore_robots) * (minute - 1));
        state.clay = state.clay.min(
            obsidian_clay_cost + (obsidian_clay_cost - state.clay_robots) * (minute - 1));
        state.obsidian = state.obsidian.min(
            geode_obsidian_cost + (geode_obsidian_cost - state.obsidian_robots) * (minute - 1),);
        if seen.contains(&state) {
            continue;
        }

        seen.insert(state);
        if state.ore_robots < max_ore_cost && state.ore >= ore_cost {
            let mut new_state = state.collect();
            new_state.ore -= ore_cost;
            new_state.ore_robots += 1;
            queue.push_back((minute - 1, new_state));
        }
        if state.clay_robots < obsidian_clay_cost && state.ore >= clay_cost {
            let mut new_state = state.collect();
            new_state.ore -= clay_cost;
            new_state.clay_robots += 1;
            queue.push_back((minute - 1, new_state));
        }
        if state.obsidian_robots < geode_obsidian_cost
            && state.ore >= obsidian_ore_cost
            && state.clay >= obsidian_clay_cost
        {
            let mut new_state = state.collect();
            new_state.ore -= obsidian_ore_cost;
            new_state.clay -= obsidian_clay_cost;
            new_state.obsidian_robots += 1;
            queue.push_back((minute - 1, new_state));
        }
        if state.ore >= geode_ore_cost && state.obsidian >= geode_obsidian_cost {
            let mut new_state = state.collect();
            new_state.ore -= geode_ore_cost;
            new_state.obsidian -= geode_obsidian_cost;
            new_state.geode_robots += 1;
            queue.push_back((minute - 1, new_state));
        }
        queue.push_back((minute - 1, state.collect()));
    }
    max_geodes
}

fn main() {
    let input = include_str!("../input.txt");

    let mut blueprints = vec![];
    for line in input.trim().lines() {
        let capture = REGEX.captures(line).unwrap();
        let blueprint = (
            capture[1].parse().unwrap(),
            capture[2].parse().unwrap(),
            capture[3].parse().unwrap(),
            capture[4].parse().unwrap(),
            capture[5].parse().unwrap(),
            capture[6].parse().unwrap(),
            capture[7].parse().unwrap(),
        );
        blueprints.push(blueprint);
    }

    let part1: usize = blueprints.iter()
        .map(|blueprint| blueprint.0 * max_geodes(*blueprint, 24))
        .sum();
    println!("{part1}");

    let part2: usize = blueprints.iter().take(3)
        .map(|blueprint| max_geodes(*blueprint, 32))
        .product();
    println!("{part2}");
}
