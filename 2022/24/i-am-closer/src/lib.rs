use std::fmt::Display;

pub fn challenge_24(input: &str) -> (usize, usize) {
    let (mut blizzards, width, height) = parse_data(input);

    let map_template = (0..height)
        .map(|y| {
            (0..width)
                .map(move |x| {
                    if x == 1 && y == 0 || x == width - 2 && y == height - 1 {
                        Grid::Space
                    } else if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                        Grid::Wall
                    } else {
                        Grid::Space
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let entry1 = (1, 0);
    let entry2 = (width - 2, height - 1);

    let part1 = travel(&map_template, &mut blizzards, entry1, entry2);
    let part2 = part1
        + travel(&map_template, &mut blizzards, entry2, entry1)
        + travel(&map_template, &mut blizzards, entry1, entry2);

    (part1, part2)
}

fn travel(
    map_template: &Vec<Vec<Grid>>,
    blizzards: &mut Vec<Blizzard>,
    start: (usize, usize),
    goal: (usize, usize),
) -> usize {
    let width = map_template[0].len();
    let height = map_template.len();

    let mut old = map_template.clone();
    old[start.1][start.0] = Grid::Stepped;
    let mut next;

    for loop_count in 1..usize::MAX {
        next = map_template.clone();
        blizzards.iter_mut().for_each(|b| {
            b.proceed(width, height);
            next[b.y][b.x] = Grid::Blizzard;
        });

        next.iter_mut().enumerate().for_each(|(y, row)| {
            row.iter_mut()
                .enumerate()
                .filter(|(x, grid)| {
                    grid.is_space()
                        && [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)]
                            .iter()
                            .filter_map(|(dx, dy)| {
                                old.get((y as i32 + dy) as usize)
                                    .and_then(|line| line.get((*x as i32 + dx) as usize))
                            })
                            .any(|grid| grid.is_stepped())
                })
                .for_each(|(_, grid)| {
                    *grid = Grid::Stepped;
                })
        });

        if let Grid::Stepped = next[goal.1][goal.0] {
            return loop_count;
        }
        old = next;
    }
    usize::MAX
}

fn parse_data(input: &str) -> (Vec<Blizzard>, usize, usize) {
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();
    let blizzards = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '>' => Some(Blizzard { x, y, dx: 1, dy: 0 }),
                '<' => Some(Blizzard {
                    x,
                    y,
                    dx: -1,
                    dy: 0,
                }),
                'v' => Some(Blizzard { x, y, dx: 0, dy: 1 }),
                '^' => Some(Blizzard {
                    x,
                    y,
                    dx: 0,
                    dy: -1,
                }),
                _ => None,
            })
        })
        .collect::<Vec<_>>();

    (blizzards, width, height)
}

#[derive(Debug)]
struct Blizzard {
    x: usize,
    y: usize,
    dx: i32,
    dy: i32,
}

impl Blizzard {
    fn proceed(&mut self, width: usize, height: usize) {
        self.x = ((self.x as i32 + self.dx - 1).rem_euclid(width as i32 - 2) + 1) as usize;
        self.y = ((self.y as i32 + self.dy - 1).rem_euclid(height as i32 - 2) + 1) as usize;
    }
}

#[derive(Debug, Clone, Copy)]
enum Grid {
    Space,
    Wall,
    Blizzard,
    Stepped,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            &Grid::Space => String::from("."),
            &Grid::Wall => String::from("#"),
            &Grid::Blizzard => String::from("%*"),
            &Grid::Stepped => format!("E"),
        };

        write!(f, "{}", s)
    }
}

impl Grid {
    fn is_space(&self) -> bool {
        if let Self::Space = *self {
            true
        } else {
            false
        }
    }

    fn is_stepped(&self) -> bool {
        match *self {
            Self::Stepped => true,
            _ => false,
        }
    }
}
