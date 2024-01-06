pub fn solve(lines: &Vec<&str>) -> u64 {
    let schematic = closer_d03::make_schematic(lines);

    schematic
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(x, c)| if *c == '*' { Some((y, x)) } else { None })
                .collect::<Vec<_>>()
        })
        .map(|(y, x)| {
            let items = process_line(&schematic[y - 1], x)
                .into_iter()
                .chain(process_line(&schematic[y], x).into_iter())
                .chain(process_line(&schematic[y + 1], x).into_iter())
                .collect::<Vec<_>>();

            if items.len() == 2 {
                items[0] * items[1]
            } else {
                0
            }
        })
        .sum()
}

fn process_line(line: &Vec<char>, pos: usize) -> Vec<u64> {
    match (
        line[pos - 1].is_digit(10),
        line[pos].is_digit(10),
        line[pos + 1].is_digit(10),
    ) {
        (_, true, _) => vec![get_flood_number(line, pos)],
        (true, _, true) => vec![
            get_flood_number(line, pos - 1),
            get_flood_number(line, pos + 1),
        ],
        (true, _, _) => vec![get_flood_number(line, pos - 1)],
        (_, _, true) => vec![get_flood_number(line, pos + 1)],
        _ => vec![],
    }
}

fn get_flood_number(line: &Vec<char>, pos: usize) -> u64 {
    let mut left = pos;
    let mut right = pos;

    while line[left - 1].is_digit(10) {
        left -= 1;
    }
    while line[right + 1].is_digit(10) {
        right += 1;
    }

    line[left..=right]
        .iter()
        .fold(0, |acc, d| acc * 10 + d.to_digit(10).unwrap() as u64)
}
