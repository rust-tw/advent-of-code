pub fn solve(lines: &Vec<&str>) -> u64 {
    let schematic = closer_d03::make_schematic(lines);

    schematic
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .fold((None, 0, 0), |(start, single, sum), (x, c)| {
                    if let Some(digit) = c.to_digit(10) {
                        if let None = start {
                            (Some(x), digit as u64, sum)
                        } else {
                            (start, single * 10 + (digit as u64), sum)
                        }
                    } else {
                        if let None = start {
                            (start, single, sum)
                        } else {
                            let pos = start.unwrap();
                            let included = schematic[y - 1][(pos - 1)..(x + 1)]
                                .iter()
                                .chain(schematic[y][(pos - 1)..pos].iter())
                                .chain(schematic[y][x..(x + 1)].iter())
                                .chain(schematic[y + 1][(pos - 1)..(x + 1)].iter())
                                .any(|c| *c != '.');
                            if included {
                                (None, 0, sum + single)
                            } else {
                                (None, 0, sum)
                            }
                        }
                    }
                })
                .2  // sum of a line
        })
        .sum()
}
