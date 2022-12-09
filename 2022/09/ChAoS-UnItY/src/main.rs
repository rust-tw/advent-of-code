use std::collections::HashSet;

trait Position<T>
where
    T: Position<T>,
{
    fn offset(&self, pos: T) -> Self;

    fn offset_x(&self, x: i32) -> Self;

    fn offset_y(&self, y: i32) -> Self;
}

trait Knot<P>
where
    P: Position<P>,
{
    fn move_head(&self, direction: char) -> P;

    fn follow_head(&self, tail: &P) -> P;
}

impl Position<(i32, i32)> for (i32, i32) {
    fn offset(&self, pos: (i32, i32)) -> Self {
        self.offset_x(pos.0).offset_y(pos.1)
    }

    fn offset_x(&self, x: i32) -> Self {
        (self.0 + x, self.1)
    }

    fn offset_y(&self, y: i32) -> Self {
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

    fn follow_head(&self, head: &(i32, i32)) -> (i32, i32) {
        match self {
            (x, y) if head.0 == *x && (head.1 - y).abs() > 1 => {
                self.offset_y((head.1 - y).signum())
            }
            (x, y) if head.1 == *y && (head.0 - x).abs() > 1 => {
                self.offset_x((head.0 - x).signum())
            }
            (x, y) if (head.1 - y).abs() + (head.0 - x).abs() >= 3 => {
                self.offset(((head.0 - x).signum(), (head.1 - y).signum()))
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
        for _ in 0 .. *step {
            knots[0] = knots[0].move_head(*direction);

            for i in 1 .. SIZE {
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
