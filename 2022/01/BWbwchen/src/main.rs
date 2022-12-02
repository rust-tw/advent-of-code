fn main() {
    let mut input: Vec<u32> = include_str!("../input.txt")
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|x| x.lines().map(|item| item.parse::<u32>().unwrap()).sum())
        .collect();
    println!("case1: {}", input.iter().max().unwrap_or(&0u32));

    input.sort();
    println!("case2: {}", input.iter().rev().take(3).sum::<u32>());
}
