fn main() {
    let content = include_str!("../Day04.txt");
    part1(content);
    part2(content);
}

fn part1(content: &str) {
    const DIRECTION_VECTORS: [[isize; 2]; 8] = [
        [1, 0],
        [1, 1],
        [0, 1],
        [-1, 1],
        [-1, 0],
        [-1, -1],
        [0, -1],
        [1, -1],
    ];

    let lines: Vec<Vec<char>> = content
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();
    let x_len = lines[0].len();
    let y_len = lines.len();
    let mut appeared = 0;

    fn slice(
        x: usize,
        y: usize,
        &[xv, yv]: &[isize; 2],
        x_len: usize,
        y_len: usize,
        lines: &Vec<Vec<char>>,
    ) -> Option<String> {
        match xv {
            -1 => {
                if x < 3 {
                    return None;
                }
            }
            1 => {
                if x > x_len - 4 {
                    return None;
                }
            }
            _ => {}
        }

        match yv {
            -1 => {
                if y < 3 {
                    return None;
                }
            }
            1 => {
                if y > y_len - 4 {
                    return None;
                }
            }
            _ => {}
        }

        let mut str_builder = String::with_capacity(4);

        for scale in 0..4 {
            str_builder.push(
                lines[(y as isize + yv * scale as isize) as usize]
                    [(x as isize + xv * scale as isize) as usize],
            );
        }

        Some(str_builder)
    }

    for x in 0..x_len {
        for y in 0..y_len {
            for vector in DIRECTION_VECTORS {
                if let Some(s) = slice(x, y, &vector, x_len, y_len, &lines) {
                    if s.as_str() == "XMAS" {
                        appeared += 1;
                    }
                }
            }
        }
    }

    println!("Part 1: {appeared}");
}

fn part2(content: &str) {
    const PATTERNS: [[&str; 3]; 4] = [
        [
            "M M",
            " A ",
            "S S"
        ],
        [
            "S M",
            " A ",
            "S M",
        ],
        [
            "S S",
            " A ",
            "M M"
        ],
        [
            "M S",
            " A ",
            "M S"
        ]
    ];
    let lines: Vec<Vec<char>> = content
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();
    let x_len = lines[0].len();
    let y_len = lines.len();
    let mut appeared = 0;

    fn validate_pat(x: usize, y: usize, lines: &Vec<Vec<char>>) -> bool {
        'pattern: for pat in PATTERNS {
            for (y_offset, &line) in pat.iter().enumerate() {
                for (x_offset, c) in line.chars().enumerate() {
                    if c == ' ' {
                        continue;
                    }

                    if c != lines[y + y_offset][x + x_offset] {
                        continue 'pattern;
                    }
                }
            }

            return true;
        }

        false
    }

    for x in 0..x_len - 2 {
        for y in 0..y_len - 2 {
            if validate_pat(x, y, &lines) {
                appeared += 1;
            }
        }
    }

    println!("Part 2: {appeared}");
}
