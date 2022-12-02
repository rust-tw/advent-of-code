use aoc_q2::solution::Solution;

fn main() {
    let input = include_str!("../inputs.txt");

    let solution =
        Solution::new_from_str(input).expect("Failed to parse input to solution structure");

    println!(
        "Answer #1: {}",
        solution.point().expect("Failed to construct ResponseList")
    );
    println!(
        "Answer #2: {}",
        solution
            .find_best_solution()
            .expect("Failed to construct ResponseStrategyMap")
            .point()
    );
}
