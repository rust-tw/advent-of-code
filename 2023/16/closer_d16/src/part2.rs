use closer_d16::*;

pub fn solve(lines: &Vec<&str>) -> usize {
    let map = Map::with_lines(lines);

    [
        (0, map.height, 0, 1, Dir::Right),
        (0, 1, 0, map.width, Dir::Down),
        (0, map.height, map.width - 1, map.width, Dir::Left),
        (map.height - 1, map.height, 0, map.width, Dir::Up),
    ]
    .into_iter()
    .map(|(y0, y1, x0, x1, dir)| {
        (y0..y1)
            .map(|y| {
                (x0..x1)
                    .map(|x| map.clone().traverse((y, x), dir).energized_count())
                    .max()
                    .unwrap_or(0)
            })
            .max()
            .unwrap_or(0)
    })
    .max()
    .unwrap_or(0)
}
