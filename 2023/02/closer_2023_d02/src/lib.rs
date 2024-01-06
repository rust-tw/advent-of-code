pub mod part1;
pub mod part2;

fn handle_line(line: &str) -> (u64, u64, u64, u64) {
    let mut parts = line.split(":");

    let title = parts.next().unwrap();
    let procedure = parts.next().unwrap().trim();

    let game_id = title
        .split_ascii_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .parse()
        .unwrap();

    let mut tokens = procedure.split(&[' ', ',', ';']).filter(|s| !s.is_empty());

    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    while let Some(s) = tokens.next() {
        let count = s.parse::<u64>().unwrap();
        let color = tokens.next().unwrap();
        match color {
            "red" => red = red.max(count),
            "green" => green = green.max(count),
            "blue" => blue = blue.max(count),
            _ => (),
        };
    }

    (game_id, red, green, blue)
}
