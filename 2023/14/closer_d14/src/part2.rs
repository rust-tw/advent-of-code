use closer_d14::{Direction, Platform};
use std::collections::HashMap;

pub fn solve(lines: &Vec<&str>) -> u64 {
    let mut platform = Platform::parse(lines);
    let mut history = HashMap::new();
    let target_cycles = 1000000000;

    for idx in 1..=target_cycles {
        let orig = platform.clone();

        platform = platform
            .tilt(Direction::North)
            .tilt(Direction::West)
            .tilt(Direction::South)
            .tilt(Direction::East);

        let diff_list = platform.diff_rocks(&orig);
        match history.get(&diff_list) {
            Some((first_idx, _)) => {
                let cycle_length = idx - first_idx;
                let target = (target_cycles - first_idx) % cycle_length + first_idx;
                return history.into_values().find(|(i, _)| *i == target).unwrap().1;
            }
            _ => {
                history.insert(diff_list, (idx, platform.eval()));
            }
        }
    }

    platform.eval()
}
