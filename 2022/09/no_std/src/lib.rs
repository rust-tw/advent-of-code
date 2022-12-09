#![no_std]

use scapegoat::SgSet;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn positions<const N: usize>(
    motions: &[(Direction, usize)],
    mut rope: [(isize, isize); N],
) -> usize {
    let mut visited: SgSet<(isize, isize), 10000> = SgSet::new();
    let len = rope.len();

    visited.insert((0, 0));
    for (direction, steps) in motions {
        (0..*steps).for_each(|_| {
            match direction {
                Direction::Up => rope[0].1 += 1,
                Direction::Down => rope[0].1 -= 1,
                Direction::Left => rope[0].0 -= 1,
                Direction::Right => rope[0].0 += 1,
            };

            (1..len).for_each(|i| {
                let (x, y) = (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1);
                if x.abs() > 1 || y.abs() > 1 {
                    rope[i].0 += x.signum();
                    rope[i].1 += y.signum();
                }
            });
            visited.insert(rope[len - 1]);
        });
    }

    visited.len()
}
