use std::io::BufReader;

use aoc_2022_d4::assignment::Assignments;

fn main() {
    let mut args = std::env::args();

    if args.len() != 2 {
        println!(
            "Usage: {} [input_file]",
            args.next().expect("Failed to get the filename.")
        );
        std::process::exit(1);
    }

    let filename = args.nth(1).expect("Failed to get the filename.");
    let file = std::fs::File::open(filename).expect("Failed to open the file.");
    let reader = BufReader::new(file);
    let assignments = Assignments::from_reader(reader).expect("Failed to construct assignments");

    println!("Q1 Answer: {}", assignments.count_subset());
    println!("Q2 Answer: {}", assignments.count_overlap());
}
