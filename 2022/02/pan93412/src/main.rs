use aoc_d2::solution::Solution;

fn main() {
    let input = include_str!("../inputs.txt");

    let solution =
        Solution::new_from_str(input).expect("Failed to parse input to solution structure");

    println!(
        "Q1 Answer: {}",
        solution.point().expect("Failed to construct ResponseList")
    );
    println!(
        "Q2 Answer: {}",
        solution
            .find_best_solution()
            .expect("Failed to construct ResponseStrategyMap")
            .point()
    );
}
