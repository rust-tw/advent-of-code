fn main() {
    let content = include_str!("../Day01.txt");
    part1(content);
    part2(content);
}

fn part1(content: &str) {
    let result = content
        .split("\n")
        .map(|line| {
            let mut numerics = line.bytes()
                .filter(u8::is_ascii_digit)
                .map(|byte| (byte - b'0') as u32);

            // Note: case (None, Some) is probably impossible to reach
            match (numerics.next(), numerics.next_back()) {
                (Some(first), Some(last)) => first * 10 + last,
                (Some(first), None) => first * 10 + first,
                (None, Some(last)) => last * 10 + last,
                (None, None) => panic!("Corrupted input file"),
            }
        })
        .sum::<u32>();

    println!("Part 1: {result}");
}

fn part2(content: &str) {
    static NUMERIC_LITERALS: [(&'static str, u32); 9] = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let result = content
        .split("\n")
        .map(|line| {
            let bytes = line.as_bytes();
            let mut numerics = vec![];

            for i in 0..bytes.len() {
                if bytes[i].is_ascii_digit() {
                    numerics.push((bytes[i] - b'0') as u32);
                    continue;
                }

                for (target, numeric) in &NUMERIC_LITERALS {
                    if (&bytes[i..]).starts_with(&target.as_bytes()) {
                        numerics.push(*numeric);
                        break;
                    }
                }
            }

            let first = numerics.first().unwrap() * 10;
            let last = numerics.last().unwrap();

            first + last
        })
        .sum::<u32>();

        println!("Part 2: {result}");
}
