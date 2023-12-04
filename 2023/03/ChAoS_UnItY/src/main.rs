use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct Span {
    start: Pos,
    end: Pos,
}

impl Span {
    fn inside_pos(&self, (col, row): Pos) -> bool {
        let &Self {
            start: (start_col, start_row),
            end: (end_col, end_row),
        } = self;

        start_col <= col && col <= end_col && start_row <= row && row < end_row
    }
}

type Pos = (usize, usize);

fn main() {
    let content = include_str!("../Day03.txt");
    let lines = resolve_input(content);
    part1(&lines);
    part2(&lines)
}

fn resolve_input(input: &str) -> Vec<&[u8]> {
    input.split("\n").map(str::as_bytes).collect()
}

fn generate_adjacent_poses(
    col_pos: usize,
    col_len: usize,
    row_pos: usize,
    row_len: usize,
    part_len: usize,
) -> Vec<Pos> {
    let mut poses = vec![];

    let up_bound = col_pos.checked_sub(1).unwrap_or(col_pos);
    let left_bound = row_pos.checked_sub(1).unwrap_or(row_pos);
    let right_bound = if row_pos + part_len + 1 >= row_len {
        row_len
    } else {
        row_pos + part_len + 1
    };
    let down_bound = if col_pos + 1 >= col_len {
        col_pos
    } else {
        col_pos + 1
    };

    for col_offset in up_bound..=down_bound {
        for row_offset in left_bound..right_bound {
            if col_offset == col_pos && row_offset >= row_pos && row_offset < right_bound - 1 {
                continue;
            }

            poses.push((col_offset, row_offset));
        }
    }

    poses
}

fn part1(lines: &[&[u8]]) {
    let mut result = 0;
    let col_len = lines.len();
    let row_len = lines[0].len();

    for col in 0..col_len {
        let mut row = 0;

        while row < row_len {
            if lines[col][row].is_ascii_digit() {
                let part_len = lines[col][row..]
                    .iter()
                    .take_while(|b| b.is_ascii_digit())
                    .count();
                let part_number = lines[col][row..row + part_len]
                    .iter()
                    .map(|&b| (b - b'0') as u32)
                    .fold(0, |acc, e| acc * 10 + e);
                let has_symbol = generate_adjacent_poses(col, col_len, row, row_len, part_len)
                    .iter()
                    .map(|&(target_col, target_row)| lines[target_col][target_row])
                    .any(|b| !b.is_ascii_digit() && b != b'.');

                if has_symbol {
                    result += part_number;
                }

                row += part_len
            } else {
                row += 1;
            }
        }
    }

    println!("Part 1: {result}");
}

fn part2(lines: &[&[u8]]) {
    let mut result = 0;
    let mut number_spans = vec![];
    let mut asterisk_poses = vec![];
    let col_len = lines.len();
    let row_len = lines[0].len();

    for col in 0..col_len {
        let mut row = 0;

        while row < row_len {
            if lines[col][row].is_ascii_digit() {
                let part_len = lines[col][row..]
                    .iter()
                    .take_while(|b| b.is_ascii_digit())
                    .count();
                let part_number = lines[col][row..row + part_len]
                    .iter()
                    .map(|&b| (b - b'0') as u32)
                    .fold(0, |acc, e| acc * 10 + e);
                let span = Span {
                    start: (col, row),
                    end: (col, row + part_len),
                };

                number_spans.push((part_number, span));
                row += part_len;
            } else {
                if lines[col][row] == b'*' {
                    asterisk_poses.push((col, row));
                }

                row += 1;
            }
        }
    }

    for (asterisk_col, asterisk_row) in asterisk_poses {
        let surrounding_numbers =
            generate_adjacent_poses(asterisk_col, col_len, asterisk_row, row_len, 1)
                .iter()
                .flat_map(|pos| {
                    number_spans
                        .iter()
                        .filter(|(_, span)| span.inside_pos(*pos))
                        .map(|&(number, _)| number)
                        .collect::<Vec<_>>()
                })
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();

        if surrounding_numbers.len() == 2 {
            result += surrounding_numbers.into_iter()
                .fold(1u32, u32::wrapping_mul);
        }
    }

    println!("Part 2: {result}");
}
