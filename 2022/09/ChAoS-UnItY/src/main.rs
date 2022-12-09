use std::collections::HashSet;

trait Position<T>
where
    T: Position<T>,
{
    type Base;

    fn x(&self) -> Self::Base;

    fn y(&self) -> Self::Base;

    fn offset<P>(&self, pos: P) -> Self
    where
        P: Position<P, Base = Self::Base>;

    fn offset_x(&self, x: Self::Base) -> Self;

    fn offset_y(&self, y: Self::Base) -> Self;
}

trait Knot<T>
where
    T: Position<T>,
{
    fn move_head(&self, direction: char) -> T;

    fn follow_head<P>(&self, head: &P) -> T
    where
        P: Position<P, Base = T::Base>;
}

impl Position<(i32, i32)> for (i32, i32) {
    type Base = i32;

    fn x(&self) -> Self::Base {
        self.0
    }

    fn y(&self) -> Self::Base {
        self.1
    }

    fn offset<P>(&self, pos: P) -> Self
    where
        P: Position<P, Base = Self::Base>,
    {
        self.offset_x(pos.x()).offset_y(pos.y())
    }

    fn offset_x(&self, x: Self::Base) -> Self {
        (self.0 + x, self.1)
    }

    fn offset_y(&self, y: Self::Base) -> Self {
        (self.0, self.1 + y)
    }
}

impl Knot<(i32, i32)> for (i32, i32) {
    fn move_head(&self, direction: char) -> (i32, i32) {
        match direction {
            'U' => self.offset_y(1),
            'D' => self.offset_y(-1),
            'L' => self.offset_x(-1),
            'R' => self.offset_x(1),
            _ => self.clone(),
        }
    }

    fn follow_head<P>(&self, head: &P) -> (i32, i32)
    where
        P: Position<P, Base = i32>,
    {
        match self {
            (x, y) if head.x() == *x && (head.y() - y).abs() > 1 => {
                self.offset_y((head.y() - y).signum())
            }
            (x, y) if head.y() == *y && (head.x() - x).abs() > 1 => {
                self.offset_x((head.x() - x).signum())
            }
            (x, y) if (head.y() - y).abs() + (head.x() - x).abs() >= 3 => {
                self.offset(((head.x() - x).signum(), (head.y() - y).signum()))
            }
            _ => self.clone(),
        }
    }
}

fn main() {
    let data = include_str!("../Day09.txt");
    let processed_data = process_data(data);

    println!("{}", part1(&processed_data));
    println!("{}", part2(&processed_data));
}

fn part1(data: &Vec<(char, usize)>) -> usize {
    traverse::<2>(data)
}

fn part2(data: &Vec<(char, usize)>) -> usize {
    traverse::<10>(data)
}

fn traverse<const SIZE: usize>(data: &Vec<(char, usize)>) -> usize {
    let mut traversed_positions = HashSet::<(i32, i32)>::new();
    let mut knots = [(0, 0); SIZE];

    for (direction, step) in data {
        for _ in 0..*step {
            knots[0] = knots[0].move_head(*direction);

            for i in 1..SIZE {
                knots[i] = knots[i].follow_head(&knots[i - 1]);
            }

            traversed_positions.insert(knots[SIZE - 1]);
        }
    }

    traversed_positions.len()
}

fn process_data(data: &'static str) -> Vec<(char, usize)> {
    data.replace("\r\n", "\n")
        .split("\n")
        .map(|line| {
            line.split_once(" ")
                .map(|(direction, step)| {
                    (
                        direction.chars().next().unwrap(),
                        step.parse::<usize>().unwrap(),
                    )
                })
                .unwrap()
        })
        .collect()
}
