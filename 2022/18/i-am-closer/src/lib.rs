use std::collections::HashSet;

const SPACE_SIZE: usize = 20;

pub fn challenge_18(lines: Vec<&str>) -> (usize, usize) {
    let data = lines
        .iter()
        .map(|line| {
            line.split(',')
                .map(|token| token.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[usize; 3]>>();

    let mut dimensions = vec![vec![HashSet::new(); 20]; 3];

    for &[x, y, z] in &data {
        dimensions[0][x].insert((y << 8) | z);
        dimensions[1][y].insert((x << 8) | z);
        dimensions[2][z].insert((x << 8) | y);
    }

    let part1 = dimensions
        .iter()
        .map(|dim| {
            let temp: usize = dim
                .windows(2)
                .map(|pair| {
                    pair[0].len() + pair[1].len() - (pair[0].intersection(&pair[1]).count()) * 2
                })
                .sum();
            dim[0].len() + dim[dim.len() - 1].len() + temp
        })
        .sum();

    // Initial states of `space`:
    //   - `u8::MAX`: occupied by lava
    //   - `u8::MAX - 1`: grids that has not been checked
    let mut space = [[[u8::MAX - 1; SPACE_SIZE + 2]; SPACE_SIZE + 2]; SPACE_SIZE + 2];
    for &[x, y, z] in &data {
        space[x + 1][y + 1][z + 1] = u8::MAX;
    }

    let mut q = Vec::new();
    q.push((0, 0, 0));
    while let Some((x, y, z)) = q.pop() {
        for [x, y, z] in get_neighbors(x, y, z, SPACE_SIZE + 2) {
            if space[x][y][z] != u8::MAX && space[x][y][z] != 0 {
                space[x][y][z] = 0;
                q.push((x, y, z));
            }
        }
    }
    // Final states of `space`:
    //   - `u8::MAX`: occupied by lava
    //   - `u8::MAX - 1`: grids blocked by lava
    //   - `0`: grids that can be reached by air

    let part2 = (0..(SPACE_SIZE + 2))
        .map(|x| {
            (0..(SPACE_SIZE + 2))
                .map(|y| {
                    (0..(SPACE_SIZE + 2))
                        .filter(|z| space[x][y][*z] == 0)
                        .map(|z| {
                            get_neighbors(x, y, z, SPACE_SIZE + 2)
                                .into_iter()
                                .filter(|&[x, y, z]| space[x][y][z] == 255)
                                .count()
                        })
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    (part1, part2)
}

fn get_neighbors(x: usize, y: usize, z: usize, boundary: usize) -> Vec<[usize; 3]> {
    let (x, y, z) = (x as i64, y as i64, z as i64);
    [
        [x + 1, y, z],
        [x - 1, y, z],
        [x, y + 1, z],
        [x, y - 1, z],
        [x, y, z + 1],
        [x, y, z - 1],
    ]
    .iter()
    .filter(|a| a.iter().all(|x| *x >= 0 && *x < boundary as i64))
    .map(|[x, y, z]| [*x as usize, *y as usize, *z as usize])
    .collect::<Vec<_>>()
}
