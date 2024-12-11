fn main() {
    // read content from file input.txt
    let file_path = "input.txt"; // change this to the path of your file
    let contents = std::fs::read_to_string(file_path).expect("File Read error");

    // one report per line
    // each report is a list of levels
    let reports = contents.lines().map(|x| {
        x.split(" ").map(|x| x.parse().unwrap()).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();

    /* Part 1 */

    let part1_answer = reports.iter().map(|report| {
        // If "safe", both of these should be true:
        // 1. The levels are either all increasing or all decreasing.
        // 2. Any two adjacent levels differ by at least one and at most three.

        // sorted => increasing
        // sorted after reverse => decreasing
        if is_sorted(report) {
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

    /* Part 2 */

    let part2_answer = reports.iter().filter(|report| {
        // If "safe", both of these should be true:
        // 1. The levels are either all increasing or all decreasing.
        // 2. Any two adjacent levels differ by at least one and at most three.

        let mut is_safe = false;
        'outer: for i in 0..report.len() {
            let mut clone_report = (*report).to_owned();
            clone_report.remove(i);

            // Step 1: check order first
            if !is_sorted(&clone_report) {
                continue;
            }

            // Step 2: check the distance between each level
            let mut prev_level = clone_report[0];
            for j in clone_report.iter().skip(1) {
                if j.abs_diff(prev_level) > 3 || j.eq(&prev_level) {
                    // If any two adjacent levels differ by more than three or equal, the report is not safe.
                    // We can jump to next round
                    continue 'outer;
                }
                prev_level = *j;
            }

            // pass all tests
            is_safe = true;
            break;
        }
        is_safe
    }).count();

    println!("Part 2: {}", part2_answer);
}

/// Determine whether the report is sorted (increasing or decreasing)
///
/// * sorted => increasing
/// * sorted after reverse => decreasing
fn is_sorted(report: &Vec<u32>) -> bool {
    report.is_sorted() || report.iter().rev().collect::<Vec<_>>().is_sorted()
}
