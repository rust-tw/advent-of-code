use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const DIR: [(isize, isize); 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn main() {
    let input = include_str!("../input.txt");
    let mut positions = input
        .lines()
        .enumerate()
        .flat_map(|(x, l)| {
            l.bytes()
                .enumerate()
                .filter(|&(_, b)| b == b'#')
                .map(move |(y, _)| (x as isize, y as isize))
        })
        .collect::<HashSet<_>>();

    let mut part1 = 0;
    let part2;
    let mut round = 0;
    loop {
        let mut proposals = HashMap::new();
        for &(x, y) in &positions {
            let next = DIR
                .iter()
                .map(|(i, j)| positions.contains(&(x + i, y + j)))
                .collect::<Vec<_>>();
            if next.iter().all(|i| !i) {
                continue;
            }
            let propose = [
                (!next[5] && !next[6] && !next[7], (x - 1, y)),
                (!next[0] && !next[1] && !next[2], (x + 1, y)),
                (!next[2] && !next[4] && !next[7], (x, y - 1)),
                (!next[0] && !next[3] && !next[5], (x, y + 1)),
            ];
            for i in 0..4 {
                let (free, pos) = propose[(round + i) % 4];
                if free {
                    proposals
                        .entry(pos)
                        .and_modify(|v| *v = None)
                        .or_insert(Some((x, y)));
                    break;
                }
            }
        }

        let mut moved = false;
        for (pos, candidate) in proposals {
            if let Some(i) = candidate {
                moved = true;
                positions.remove(&i);
                positions.insert(pos);
            }
        }
        if !moved {
            part2 = round + 1;
            break;
        } else if round == 9 {
            part1 = empty_tiles(&positions);
        }
        round += 1;
    }
    println!("{part1}, {part2}");
}

fn empty_tiles(positions: &HashSet<(isize, isize)>) -> usize {
    let (&minx, &maxx) = positions
        .iter()
        .map(|(x, _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let (&miny, &maxy) = positions
        .iter()
        .map(|(_, y)| y)
        .minmax()
        .into_option()
        .unwrap();
    (minx..=maxx)
        .cartesian_product(miny..=maxy)
        .filter(|p| !positions.contains(p))
        .count()
}
