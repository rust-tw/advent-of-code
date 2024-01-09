use closer_d10::*;

pub fn solve(input: &str) -> i32 {
    let (mut grids, start_y, start_x) = build_grids(input);
    let max = (grids.len(), grids[0].len());

    let mut x = start_x;
    let mut y = start_y;
    let mut dir = grids[y][x].neighbors[0];
    (y, x) = dir.try_proceed(y, x, max).unwrap();
    while !grids[y][x].visited {
        grids[y][x].visited = true;

        let new_dir = grids[y][x]
            .neighbors
            .clone()
            .into_iter()
            .find(|d| *d != dir.turn_180())
            .unwrap();
        dir = new_dir;
        (y, x) = dir.try_proceed(y, x, max).unwrap();
    }

    grids
        .iter()
        .map(|row| {
            row.iter()
                .fold((false, false, 0), |(up, down, count), grid| {
                    if grid.visited {
                        (
                            up ^ grid.neighbors.contains(&Direction::Up),
                            down ^ grid.neighbors.contains(&Direction::Down),
                            count,
                        )
                    } else {
                        (up, down, if up && down { count + 1 } else { count })
                    }
                })
                .2
        })
        .sum()
}
