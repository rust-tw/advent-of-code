use std::collections::BinaryHeap;

pub fn challenge_12(lines: Vec<&str>) -> (usize, usize) {
    let height = lines.len();
    let width = lines[0].chars().count();
    let mut start = Point(0, 0);
    let mut end = Point(width - 1, height - 1);

    let levels = lines
        .iter()
        .enumerate()
        .map(|(y, &line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = Point(x, y);
                        'a' as u8
                    }
                    'E' => {
                        end = Point(x, y);
                        'z' as u8
                    }
                    ch => ch as u8,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part1 = find_shortest_path(start, end, &levels).unwrap();

    let part2 = (0..width)
        .flat_map(|x| (0..height).map(move |y| Point(x, y)))
        .filter(|point| levels[point.1][point.0] == ('a' as u8))
        .filter_map(|point| find_shortest_path(point, end, &levels))
        .min()
        .unwrap();

    (part1, part2)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point(usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    cost: usize,
    position: Point,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_shortest_path(start: Point, end: Point, levels: &Vec<Vec<u8>>) -> Option<usize> {
    // Dijkstra's shortest path algorithm.
    // Modified from https://doc.rust-lang.org/std/collections/binary_heap/index.html.
    let mut distances = vec![vec![usize::MAX; levels[0].len()]; levels.len()];
    distances[start.1][start.0] = 0;

    let mut q = BinaryHeap::new();
    q.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = q.pop() {
        if position == end {
            return Some(cost);
        }
        if cost > distances[position.1][position.0] {
            continue;
        }
        for next in get_walkable_neighbors(levels, &position) {
            let next_cost = cost + 1;
            if next_cost < distances[next.1][next.0] {
                distances[next.1][next.0] = next_cost;
                q.push(State{cost: next_cost, position: next});
            }
        }
    }

    None
}

fn get_walkable_neighbors(levels: &Vec<Vec<u8>>, position: &Point) -> Vec<Point> {
    let x = position.0 as i64;
    let y = position.1 as i64;
    let w = levels[0].len() as i64;
    let h = levels.len() as i64;

    vec![
        (x - 1, y),
        (x + 1, y),
        (x, y - 1),
        (x, y + 1),
    ]
    .into_iter()
    .filter(|&(new_x, new_y)| {
        new_x >= 0
            && new_y >= 0
            && new_x < w
            && new_y < h
            && levels[new_y as usize][new_x as usize] <= (levels[position.1][position.0] + 1)
    })
    .map(|(new_x, new_y)| Point(new_x as usize, new_y as usize))
    .collect()
}
