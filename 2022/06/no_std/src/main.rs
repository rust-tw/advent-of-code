use no_std::find_marker;

fn main() {
    let input = include_str!("../input.txt");
    let part1 = find_marker(input, 4);
    let part2 = find_marker(input, 14);
    println!("{part1}, {part2}");
}
