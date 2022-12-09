use no_std::Direction::*;

fn main() {
    let input = include_str!("../input.txt");
    let motions = input
        .lines()
        .map(|l| {
            let (direction, steps) = l.split_once(' ').unwrap();
            let direction = match direction.as_bytes()[0] as char {
                'U' => Up,
                'D' => Down,
                'L' => Left,
                'R' => Right,
                _ => unreachable!(),
            };
            let steps = steps.parse::<usize>().unwrap();
            (direction, steps)
        })
        .collect::<Vec<_>>();

    let part1 = no_std::positions(&motions, [(0, 0); 2]);
    let part2 = no_std::positions(&motions, [(0, 0); 10]);
    println!("{part1}, {part2}");
}
