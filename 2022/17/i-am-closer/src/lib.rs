mod rock;
mod tower;

use rock::Rock;
use std::collections::HashMap;
use tower::Tower;

pub fn challenge_17(lines: Vec<&str>) -> (usize, usize) {
    let part1 = action(2022, lines[0]);
    let part2 = action(1_000_000_000_000, lines[0]);
    (part1, part2)
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Snapshot {
    rock_type: usize,
    dir_index: usize,
    top_levels: [u8; 8],
}

fn action(total_rocks: usize, ops: &str) -> usize {
    let mut tower = Tower::new();

    let mut dir = ops.chars().enumerate().cycle();

    let mut rock_count = 0;

    let mut database = HashMap::new();
    let mut heights = Vec::new();
    loop {
        if rock_count >= total_rocks {
            break tower.get_height();
        }
        heights.push(tower.get_height());

        let mut rock = Rock::new(rock_count % 5, tower.get_height() as i64 + 3);
        rock_count += 1;

        let dir_index = loop {
            let index = match dir.next() {
                Some((idx, '>')) => {
                    rock.move_right(&tower);
                    idx
                }
                Some((idx, '<')) => {
                    rock.move_left(&tower);
                    idx
                }
                _ => panic!("Noooo!!!"),
            };

            if !rock.move_down(&mut tower) {
                break index;
            }
        };

        if tower.get_height() > 8 {
            let top_levels: [u8; 8] = ((tower.get_height() - 8)..tower.get_height())
                .map(|y| tower.get_level(y))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let snapshot = Snapshot {
                rock_type: rock_count % 5,
                dir_index,
                top_levels,
            };
            match database.get(&snapshot) {
                Some(&old_rock_index) => {
                    let cycle_count = (total_rocks - rock_count) / (rock_count - old_rock_index);
                    let extra_count = (total_rocks - rock_count) % (rock_count - old_rock_index);
                    let h = tower.get_height()
                        + cycle_count * (tower.get_height() - heights[old_rock_index])
                        + heights[old_rock_index + extra_count]
                        - heights[old_rock_index];
                    break h;
                }
                None => {
                    database.insert(snapshot, rock_count);
                }
            }
        }
    }
}
