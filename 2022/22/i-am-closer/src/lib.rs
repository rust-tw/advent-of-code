mod cube;

use crate::cube::Cube;
use std::collections::VecDeque;

#[derive(Debug)]
enum Action {
    Step(i32),
    Turn(i32),
}

type Maze = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Eq)]
struct Avatar {
    x: i32,
    y: i32,
    dir: i32,
}

#[derive(Debug, Default, Clone, Copy)]
struct FaceInfo {
    x: usize,
    y: usize,
    surround: [usize; 4],
}

pub fn challenge_22(lines: Vec<&str>) -> (usize, usize) {
    let (maze, actions) = parse_data(lines);
    let (h_limits, v_limits) = calc_limits(&maze);

    let unit = h_limits
        .iter()
        .map(|&(l, h)| h - l + 1)
        .min()
        .min(v_limits.iter().map(|&(l, h)| h - l + 1).min())
        .unwrap();

    let faces = calc_faces(&maze, unit as usize);

    let part1 = solve(&maze, &actions, (h_limits[0].0 as i32, 0), |a| {
        calc_next_part1(a, &h_limits, &v_limits)
    });
    let part2 = solve(&maze, &actions, (h_limits[0].0 as i32, 0), |a| {
        calc_next_part2(a, &faces, unit)
    });

    (part1, part2)
}

fn solve<F>(maze: &Maze, actions: &Vec<Action>, start: (i32, i32), calc_next: F) -> usize
where
    F: Fn(&Avatar) -> Avatar,
{
    let mut avatar = Avatar {
        x: start.0,
        y: start.1,
        dir: 0,
    };
    let delta_x = [1, 0, -1, 0];
    let delta_y = [0, 1, 0, -1];

    for action in actions {
        match action {
            &Action::Turn(dir) => avatar.dir = (avatar.dir + dir).rem_euclid(4),
            &Action::Step(steps) => {
                for _ in 0..steps {
                    let next_avatar = {
                        let x = avatar.x + delta_x[avatar.dir as usize];
                        let y = avatar.y + delta_y[avatar.dir as usize];
                        match maze.get(y as usize).and_then(|row| row.get(x as usize)) {
                            Some(c) if *c != ' ' => Avatar { x, y, ..avatar },
                            _ => calc_next(&avatar),
                        }
                    };

                    if maze[next_avatar.y as usize][next_avatar.x as usize] != '#' {
                        avatar = next_avatar;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    ((avatar.y + 1) * 1000 + (avatar.x + 1) * 4 + avatar.dir) as usize
}

fn calc_next_part1(cur: &Avatar, h_limits: &Vec<(i32, i32)>, v_limits: &Vec<(i32, i32)>) -> Avatar {
    let (x, y) = match cur.dir {
        0 => (h_limits[cur.y as usize].0, cur.y),
        1 => (cur.x, v_limits[cur.x as usize].0),
        2 => (h_limits[cur.y as usize].1, cur.y),
        3 => (cur.x, v_limits[cur.x as usize].1),
        _ => panic!("Wrong dir..."),
    };

    Avatar { x, y, ..*cur }
}

fn calc_next_part2(cur: &Avatar, faces: &[FaceInfo; 6], unit: i32) -> Avatar {
    let unit_x = cur.x / unit;
    let unit_y = cur.y / unit;
    let src = faces
        .iter()
        .enumerate()
        .find(|(_, f)| f.x == unit_x as usize && f.y == unit_y as usize)
        .unwrap()
        .0;
    let arm = match cur.dir {
        0 => (unit - cur.y - 1).rem_euclid(unit),
        1 => cur.x - unit_x * unit,
        2 => cur.y - unit_y * unit,
        3 => (unit - cur.x - 1).rem_euclid(unit),
        _ => panic!("Wrong dir"),
    };
    let dest = faces[src].surround[cur.dir as usize];

    let dir = (faces[dest]
        .surround
        .iter()
        .enumerate()
        .find(|(_, &f)| f == src)
        .unwrap()
        .0) as i32
        + 2 % 4;

    let (x, y) = match dir {
        0 => (
            (faces[dest].x as i32) * unit,
            (faces[dest].y as i32 + 1) * unit - 1 - arm,
        ),
        1 => (
            (faces[dest].x as i32) * unit + arm,
            (faces[dest].y as i32) * unit,
        ),
        2 => (
            (faces[dest].x as i32 + 1) * unit - 1,
            (faces[dest].y as i32) * unit + arm,
        ),
        3 => (
            (faces[dest].x as i32 + 1) * unit - 1 - arm,
            (faces[dest].y as i32 + 1) * unit - 1,
        ),
        _ => panic!("Wrong dir"),
    };
    Avatar { x, y, dir }
}

fn parse_data(lines: Vec<&str>) -> (Maze, Vec<Action>) {
    let mut num = 0;
    let mut actions = vec![];

    let parts = lines.split(|line| line.is_empty()).collect::<Vec<_>>();
    let maze: Maze = parts[0]
        .iter()
        .map(|&line| line.chars().collect())
        .collect();

    for c in parts[1][0].chars() {
        match c {
            d if d.is_digit(10) => {
                num *= 10;
                num += ((d as u8) - ('0' as u8)) as i32;
            }
            'L' => {
                actions.push(Action::Step(num));
                actions.push(Action::Turn(-1));
                num = 0;
            }
            'R' => {
                actions.push(Action::Step(num));
                actions.push(Action::Turn(1));
                num = 0;
            }
            _ => panic!(""),
        }
    }

    if num != 0 {
        actions.push(Action::Step(num));
    }

    (maze, actions)
}

fn calc_limits(maze: &Maze) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let height = maze.len();
    let width = maze.iter().map(|row| row.len()).max().unwrap();

    let mut h_limit = vec![(i32::MAX, 0); height];
    let mut v_limit = vec![(i32::MAX, 0); width];

    for (y, row) in maze.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c != ' ' {
                h_limit[y].0 = h_limit[y].0.min(x as i32);
                h_limit[y].1 = h_limit[y].1.max(x as i32);
                v_limit[x].0 = v_limit[x].0.min(y as i32);
                v_limit[x].1 = v_limit[x].1.max(y as i32);
            }
        }
    }

    (h_limit, v_limit)
}

fn calc_faces(maze: &Maze, unit: usize) -> [FaceInfo; 6] {
    let mut q = VecDeque::new();
    let mut visited = [[false; 4]; 4];
    let mut faces = [FaceInfo::default(); 6];

    'outer: for y in 0..4 {
        for x in 0..4 {
            if let Some(c) = maze.get(y * unit).and_then(|row| row.get(x * unit)) {
                if *c != ' ' {
                    q.push_back((x, y, Cube::default()));
                    break 'outer;
                }
            }
        }
    }

    while let Some((x, y, cube)) = q.pop_front() {
        visited[y][x] = true;
        faces[cube.get_face() as usize] = FaceInfo {
            x,
            y,
            surround: cube.get_surround(),
        };
        for dir in 0..4 {
            let (x, y) = if dir % 2 == 0 {
                ((x as i32 - dir + 1) as usize, y)
            } else {
                (x, (y as i32 - dir + 2) as usize)
            };

            if x >= 4 || y >= 4 {
                continue;
            }

            if let Some(c) = maze.get(y * unit).and_then(|row| row.get(x * unit)) {
                if *c != ' ' && !visited[y][x] {
                    q.push_back((x as usize, y as usize, cube.turn((dir + 2) % 4)));
                }
            }
        }
    }

    faces
}
