use itertools::Itertools;

mod point;
use point::Point;

pub fn challenge_14(lines: Vec<&str>) -> (usize, usize) {
    // parse input data
    let rocks = lines
        .into_iter()
        .map(|line| {
            line.split(" -> ")
                .map(|data| {
                    let (x, y) = data
                        .split(',')
                        .map(|ord| ord.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    Point { x, y }
                })
                .collect_vec()
        })
        .collect_vec();

    // find the rock boudaries to decide how large the map should be
    let (max, min) = rocks.iter().fold(
        (Point::MIN, Point::MAX),
        |(max, min), v| {
            let (max2, min2) = v.iter().fold(
                (Point::MIN, Point::MAX),
                |(max, min), p| (max.max(p), min.min(p)),
            );
            (max.max(&max2), min.min(&min2))
        },
    );

    assert!(max.x >= 500 && min.x <= 500);

    // Height should be (max.y + 1) to fit all rocks (and +2 for part 2).
    let height = max.y + 1 + 2;
    // The minimum width should be (max.x - min.x + 1).
    // But in part 2, the sands would span according to the height.
    // We should set width to max() of above values, but accroding to the
    // input data (height * 2 - 1) is larger, we simply use it.
    let width = height * 2 - 1;

    let sand_x = width / 2;
    let sand_y = 0;
    let dx = 500 - sand_x;

    let mut map1 = vec![vec![0u8; height]; width];

    // Mark occupied grids in the map
    for rock in &rocks {
        rock.windows(2).for_each(|v| {
            if v[0].x == v[1].x {
                let start = v[0].y.min(v[1].y);
                let end = v[0].y.max(v[1].y);
                for y in start..=end {
                    map1[v[0].x - dx][y] = 1;
                }
            } else if v[0].y == v[1].y {
                let start = v[0].x.min(v[1].x);
                let end = v[0].x.max(v[1].x);
                for x in start..=end {
                    map1[x - dx][v[0].y] = 1;
                }
            }
        });
    }

    let mut map2 = map1.clone();
    for x in 0..width {
        map2[x][height - 1] = 1;
    }

    let part1 = (0..(width * height))
        .map(|_| {
            let mut sand = Point::new(sand_x, sand_y);
            loop {
                if sand.y + 1 >= height {
                    break None;
                }

                if map1[sand.x][sand.y + 1] == 0 {
                    sand.y += 1;
                } else if map1[sand.x - 1][sand.y + 1] == 0 {
                    sand.x -= 1;
                    sand.y += 1;
                } else if map1[sand.x + 1][sand.y + 1] == 0 {
                    sand.x += 1;
                    sand.y += 1;
                } else {
                    map1[sand.x][sand.y] = 1;
                    break Some(sand);
                }
            }
        })
        .take_while(|o| o.is_some())
        .count();

    let part2 = (0..(width * height))
        .map(|_| {
            let mut sand = Point::new(sand_x, sand_y);
            loop {
                if map2[sand.x][sand.y + 1] == 0 {
                    sand.y += 1;
                } else if map2[sand.x - 1][sand.y + 1] == 0 {
                    sand.x -= 1;
                    sand.y += 1;
                } else if map2[sand.x + 1][sand.y + 1] == 0 {
                    sand.x += 1;
                    sand.y += 1;
                } else {
                    map2[sand.x][sand.y] = 1;
                    break sand;
                }
            }
        })
        .take_while(|p| p.x != sand_x || p.y != sand_y)
        .count() + 1;
        // +1 for the last sand, where (p.x, p.y) == (sand_x, sand_y)
    (part1, part2)
}
