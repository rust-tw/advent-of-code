use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn get_vector(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }

    fn rotate_clockwise(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn main() {
    let content = include_str!("../Day06.txt");
    let path = part1(&content);
    part2(&content, path);
}

fn parse_data(content: &str) -> ((i32, i32), Vec<(i32, i32)>) {
    let line_iter = content.lines().enumerate();
    let mut initial_pos = (0, 0);
    let mut obstacles = Vec::new();

    for (y, line) in line_iter {
        for (x, c) in line.chars().enumerate() {
            match c {
                '^' => initial_pos = (x as i32, y as i32),
                '.' => {}
                '#' => obstacles.push((x as i32, y as i32)),
                _ => unreachable!(),
            }
        }
    }

    (initial_pos, obstacles)
}

fn part1(content: &str) -> HashSet<(i32, i32)> {
    let mut direction = Direction::Up;
    let line_cnt = content.lines().count() as i32;
    let (x_size, y_size) = (content.len() as i32 / line_cnt, line_cnt);
    let ((mut x, mut y), obstacles) = parse_data(content);
    let mut distinct_pos = HashSet::new();

    loop {
        distinct_pos.insert((x, y));
        let (mut vx, mut vy) = direction.get_vector();
        let (mut nx, mut ny) = (x + vx, y + vy);

        if obstacles.contains(&(nx, ny)) {
            direction.rotate_clockwise();
            (vx, vy) = direction.get_vector();
            (nx, ny) = (x + vx, y + vy);
        }

        if nx < 0 || nx >= x_size || ny < 0 || ny >= y_size {
            // Out of bound
            break;
        }

        (x, y) = (nx, ny);
    }

    println!("Part 1: {}", distinct_pos.len());
    distinct_pos
}

fn part2(content: &str, path: HashSet<(i32, i32)>) {
    let mut options = 0;
    let line_cnt = content.lines().count() as i32;
    let (x_size, y_size) = (content.len() as i32 / line_cnt, line_cnt);
    let ((gx, gy), obstacles) = parse_data(content);
    let mut obstacles: HashSet<(i32, i32)> = obstacles.into_iter().collect();

    for (nox, noy) in path {
        obstacles.insert((nox, noy));
        let mut turns = HashSet::new();
        let (mut x, mut y) = (gx, gy);
        let mut direction = Direction::Up;

        loop {
            let (vx, vy) = direction.get_vector();
            let (nx, ny) = (x + vx, y + vy);

            if obstacles.contains(&(nx, ny)) {
                if turns.contains(&((nx, ny), direction)) {
                    options += 1;
                    break;
                }

                turns.insert(((nx, ny), direction));
                direction.rotate_clockwise();
                continue;
            }

            if nx < 0 || nx >= x_size || ny < 0 || ny >= y_size {
                // Out of bound
                break;
            }

            (x, y) = (nx, ny);
        }

        obstacles.remove(&(nox, noy));
    }

    println!("Part 2: {options}");
}
