mod grove;

use grove::Grove;
use std::collections::HashMap;

pub fn challenge_23(input: &str) -> (i32, usize) {
    let grove = input.parse::<Grove>().unwrap();

    let part1 = solve(grove.clone(), |round, _| round >= 10, |_, grove| {
        let mut v_limits = (i32::MAX, i32::MIN);
        let mut h_limits = (i32::MAX, i32::MIN);
        let mut count = 0;
        for y in 0..grove.height() {
            for x in 0..grove.width() {
                if grove.get(x, y) == 1 {
                    count += 1;
                    v_limits = (v_limits.0.min(y), v_limits.1.max(y));
                    h_limits = (h_limits.0.min(x), h_limits.1.max(x));
                }
            }
        }
        (v_limits.1 - v_limits.0 + 1) * (h_limits.1 - h_limits.0 + 1) - count
    });

    let part2 = solve(grove, |_, moves| moves == 0, |round, _| round);

    (part1, part2)
}

fn solve<F1, F2, R>(mut grove: Grove, break_cond: F1, eval: F2) -> R
where F1: Fn(usize, usize) -> bool,
      F2: Fn(usize, &Grove) -> R
{
    let surround = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    let sides = [
        [(0, -1), (1, -1), (-1, -1)], // N
        [(0, 1), (1, 1), (-1, 1)],    // S
        [(-1, 0), (-1, 1), (-1, -1)], // W
        [(1, 0), (1, -1), (1, 1)],    // E
    ];

    let mut round = 0;
    loop {
        // First Half
        let mut plans = HashMap::new();
        for y in 0..grove.height() {
            for x in 0..grove.width() {
                if grove.get(x, y) == 1 {
                    for side in sides.iter().cycle().skip(round).take(4) {
                        if surround
                            .iter()
                            .all(|&(dx, dy)| grove.get(x + dx, y + dy) == 0)
                        {
                            continue;
                        }
                        if side.iter().all(|&(dx, dy)| grove.get(x + dx, y + dy) == 0) {
                            let new_pos = (x + side[0].0, y + side[0].1);
                            plans.entry(new_pos).or_insert(vec![]).push((x, y));
                            break;
                        }
                    }
                }
            }
        }

        // Second Half
        plans.iter().for_each(|(new, olds)| {
            if olds.len() == 1 {
                grove.set(olds[0].0, olds[0].1, 0);
                grove.set(new.0, new.1, 1);
            }
        });

        grove.check_and_grow();

        round += 1;

        if break_cond(round, plans.len()) {
            break;
        }
    }

    eval(round, &grove)
}
