use no_std::*;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    // I assume everyone's grid is 99x99
    const LEN: usize = 99;

    let grid = INPUT
        .lines()
        .map(|l| {
            l.bytes()
                .map(|b| b - b'0')
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[u8; LEN]>>();

    let (part1, part2) = scan(grid.as_slice());

    println!("{part1}, {part2}");
}
