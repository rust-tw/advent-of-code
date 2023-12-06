pub mod part1;
pub mod part2;
fn main() {
    let input = include_str!("../input.txt");
    let result1 = part1::solve(input);
    println!("result1: {result1}");
    let result2 = part2::solve(input);
    println!("result2: {result2}");
}
