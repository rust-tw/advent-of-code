use std::io::BufReader;

fn main() {
    let input = include_str!("../tests/testdata/input.2.txt");

    for _ in 0..10usize.pow(3) * 2 {
        let reader = BufReader::new(input.as_bytes());
        let assignments = aoc_2022_d4::assignment::Assignments::from_reader(reader).unwrap();
        assignments.count_subset();
        assignments.count_overlap();
    }
}
