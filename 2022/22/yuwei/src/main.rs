fn main() {
    let input = include_str!("../input.txt");
    let (map, path) = input.split_once("\n\n").unwrap();
    let map = map
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let part1 = walk(&map, path, true);
    println!("{part1}");
    let part2 = walk(&map, path, false);
    println!("{part2}");
}

const DIR: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn walk(map: &[Vec<u8>], path: &str, part1: bool) -> usize {
    let wrap = if part1 { p1 } else { p2 };
    let (mut row, mut col, mut face) = (0, 0, 0);
    while map[0][col] != b'.' {
        col += 1
    }

    let mut chars = path.chars().peekable();
    let mut s = String::new();
    while let Some(ch) = chars.next() {
        if ch.is_ascii_digit() {
            s.push(ch);
        } else {
            let n = s.parse().unwrap();
            s = String::new();
            for _ in 0..n {
                let (x, y) = DIR[face];
                match map
                    .get(row + x as usize)
                    .and_then(|i| i.get(col + y as usize))
                    .unwrap_or(&b' ')
                {
                    b'.' => (row, col) = (row + x as usize, col + y as usize),
                    b'#' => break,
                    b' ' => {
                        let (r, c, f) = wrap(map, row, col, face);
                        if map[r][c] == b'#' {
                            break;
                        }
                        (row, col, face) = (r, c, f);
                    }
                    _ => unreachable!(),
                }
            }

            match ch {
                'L' => face = (face + 3) % 4,
                'R' => face = (face + 1) % 4,
                _ => (),
            }
        }
    }
    1000 * (row + 1) + 4 * (col + 1) + [0, 1, 2, 3][face]
}

fn p1(map: &[Vec<u8>], mut r: usize, mut c: usize, f: usize) -> (usize, usize, usize) {
    let (x, y) = DIR[f];
    while *map
        .get(r - x as usize)
        .and_then(|row| row.get(c - y as usize))
        .unwrap_or(&b' ')
        != b' '
    {
        (r, c) = (r - x as usize, c - y as usize);
    }
    (r, c, f)
}

fn p2(_: &[Vec<u8>], r: usize, c: usize, f: usize) -> (usize, usize, usize) {
    let (qr, qc, nf) = match (r / 50, c / 50, f) {
        (0, 1, 3) => (3, 0, 0),
        (0, 1, 2) => (2, 0, 0),
        (0, 2, 3) => (3, 0, 3),
        (0, 2, 0) => (2, 1, 2),
        (0, 2, 1) => (1, 1, 2),
        (1, 1, 0) => (0, 2, 3),
        (1, 1, 2) => (2, 0, 1),
        (2, 0, 3) => (1, 1, 0),
        (2, 0, 2) => (0, 1, 0),
        (2, 1, 0) => (0, 2, 2),
        (2, 1, 1) => (3, 0, 2),
        (3, 0, 0) => (2, 1, 3),
        (3, 0, 1) => (0, 2, 1),
        (3, 0, 2) => (0, 1, 1),
        _ => unreachable!(),
    };
    let (x, y) = (r % 50, c % 50);
    let i = [x, 49 - y, 49 - x, y][f];
    let (nr, nc) = [(i, 0), (0, 49 - i), (49 - i, 49), (49, i)][nf];
    (qr * 50 + nr, qc * 50 + nc, nf)
}
