use closer_d10::*;
use std::collections::VecDeque;

pub fn solve(input: &str) -> i32 {
    let (mut grids, start_x, start_y) = build_grids(input);

    let width = grids[0].len();
    let height = grids.len();

    let mut queue = VecDeque::new();
    queue.push_back((start_y, start_x));

    while !queue.is_empty() {
        let (y, x) = queue.pop_front().unwrap();
        grids[y][x].visited = true;
        grids[y][x].neighbors.clone().into_iter().for_each(|dir| {
            if let Some((ny, nx)) = dir.try_proceed(y, x, (height, width)) {
                if grids[ny][nx].visited {
                    grids[y][x].distance = grids[ny][nx].distance + 1;
                } else {
                    queue.push_back((ny, nx))
                }
            }
        })
    }
    // println!("{}", format_grids(&grids));

    grids
        .iter()
        .map(|line| line.iter().map(|grid| grid.distance))
        .flatten()
        .max()
        .unwrap()
}
