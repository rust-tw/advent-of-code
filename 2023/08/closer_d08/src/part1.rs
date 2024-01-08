use closer_d08::parse_data;

pub fn solve(lines: &Vec<&str>) -> u64 {
    let (steps, map) = parse_data(lines);
    let mut cur_node = ['A', 'A', 'A'];
    let mut count = 0;
    let mut dirs = steps.iter().cycle();
    while cur_node != ['Z', 'Z', 'Z'] {
        count += 1;
        let dir = dirs.next().unwrap();
        cur_node = map.get(&cur_node).unwrap()[*dir];
    }
    count
}
