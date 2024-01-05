use closer_d04::match_count;

pub fn solve(lines: &Vec<&str>) -> u64 {
    let mut copies = vec![1; lines.len()];
    
    for (idx, line) in lines.iter().enumerate() {
        let match_count = match_count(line);
        for i in (idx + 1)..=(idx + match_count) {
            if i < copies.len() {
                copies[i] += copies[idx];
            }
        }
    }
    copies.into_iter().sum()
}
