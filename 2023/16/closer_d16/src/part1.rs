use closer_d16::*;

pub fn solve(lines: &Vec<&str>) -> usize {
    Map::with_lines(lines)
        .traverse((0, 0), Dir::Right)
        .energized_count()
}
