fn main() {
    // read content from file input.txt
    let file_path = "input.txt"; // change this to the path of your file
    let contents = std::fs::read_to_string(file_path).expect("File Read error");

    // one report per line
    // each report is a list of levels
    let reports = contents.lines().map(|x| {
        x.split(" ").map(|x| x.parse().unwrap()).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();

    let part1_answer = reports.iter().map(|report| {
        // If "safe", both of these should be true:
        // 1. The levels are either all increasing or all decreasing.
        // 2. Any two adjacent levels differ by at least one and at most three.

        // sorted => increasing
        // sorted after reverse => decreasing
        if report.is_sorted() || report.iter().rev().collect::<Vec<_>>().is_sorted() {
            let mut prev_level = report[0];
            return report.iter().skip(1).all(|level| {
                // check level and next level differ by at least one and at most three
                if level.abs_diff(prev_level) > 3 || level.eq(&prev_level) {
                    return false;
                }
                prev_level = *level;
                true
            })
        }
        false
    }).filter(|check| *check).count();

    println!("Part 1: {}", part1_answer);
}
