use closer_d04::match_count;

pub fn solve(lines: &Vec<&str>) -> u64 {
    lines.iter().map(|line| process_line(line)).sum()
}

fn process_line(line: &str) -> u64 {
    let matchings = match_count(line);

    if matchings == 0 {
        0
    } else {
        2_u64.pow((matchings - 1) as u32)
    }
}
