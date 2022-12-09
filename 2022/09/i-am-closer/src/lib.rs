use std::collections::HashSet;

pub fn challenge_09(lines: Vec<&str>) -> (usize, usize) {
    // convers "R 4" -> "RRRR"; "D 3" -> "DDD"... etc.
    let moves = lines
        .into_iter()
        .map(|line| {
            let tokens = line.split(' ').collect::<Vec<_>>();
            tokens[0].repeat(tokens[1].parse::<usize>().unwrap())
        })
        .reduce(|accu, item| accu + &item)
        .unwrap();

    (compute_travelled_count(&moves, 2), compute_travelled_count(&moves, 10))
}

fn compute_travelled_count(moves: &str, knots_count: usize) -> usize {
    let mut travelled = HashSet::from([Position::new()]);
    let mut knots = vec![Position::new(); knots_count];
    let _ = moves
        .chars()
        .for_each(|dir| {
            knots[0] = knots[0].move_dir(dir);
            for i in 1..knots_count {
                knots[i] = knots[i].follow(&knots[i - 1]);
            }
            travelled.insert(*(knots.last().unwrap()));
        });
    travelled.len()
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

// impl std::fmt::Debug for Position {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }

impl Position {
    fn new() -> Self {
        Position { x: 0, y: 0 }
    }

    fn follow(&self, other: &Position) -> Self {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        if dx.abs() <= 1 && dy.abs() <= 1 {
            *self
        } else {
            Self {
                x: self.x + if dx == 0 {0} else {dx / dx.abs()},
                y: self.y + if dy == 0 {0} else {dy / dy.abs()},
            }
        }
    }

    fn move_dir(&self, dir: char) -> Self {
        match dir {
            'R' => Self {
                x: self.x + 1,
                ..*self
            },
            'L' => Self {
                x: self.x - 1,
                ..*self
            },
            'U' => Self {
                y: self.y - 1,
                ..*self
            },
            'D' => Self {
                y: self.y + 1,
                ..*self
            },
            _ => panic!("Unknown movement: {dir}"),
        }
    }
}
