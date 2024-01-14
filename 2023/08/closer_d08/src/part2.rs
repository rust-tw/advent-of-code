use closer_d08::parse_data;

pub fn solve(lines: &Vec<&str>) -> u64 {
    let (steps, map) = parse_data(lines);
    map.keys()
        .filter(|[_, _, c]| *c == 'A')
        .map(|mut cur_node| {
            // print!("{cur_node:?} -> ");
            let mut count = 0;
            let mut dirs = steps.iter().cycle();
            while cur_node[2] != 'Z' {
                count += 1;
                let dir = dirs.next().unwrap();
                cur_node = &map.get(cur_node).unwrap()[*dir];
            }
            // print!("{cur_node:?}; ");
            // println!("count = {}; count % steps.len() = {}", count, count % steps.len() as u64);
            count
        })
        .reduce(|acc, n| acc * n / gcd::binary_u64(acc, n))
        .unwrap()
}
