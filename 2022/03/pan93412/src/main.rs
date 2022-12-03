use aoc_2022_d3::solution::Solution;

fn main() {
    let inputs = include_str!("../inputs.txt");

    let s = Solution::new(inputs);
    println!("Q1: {}", s.solve_question_1());
    println!("Q2: {}", s.solve_question_2());
}
