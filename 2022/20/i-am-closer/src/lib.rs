pub fn challenge_20(lines: Vec<&str>) -> (i64, i64) {
    let input: Vec<_> = lines.iter().map(|s| s.parse::<i64>().unwrap()).collect();

    let part1 = solve(&input, 1, 1);
    let part2 = solve(&input, 811589153, 10);

    (part1, part2)
}

fn solve(input: &Vec<i64>, decrypt_key: i64, times: usize) -> i64 {
    let length = input.len() as i64;
    let input: Vec<_> = input.into_iter().map(|x| x * decrypt_key).collect();
    let mut pos = (0..length).collect::<Vec<_>>();

    for _ in 0..times {
        for i in 0..input.len() {
            let new_pos = ((pos[i] + input[i]) % (length - 1) + (length - 1)) % (length - 1);

            let (dir, lbound, ubound) = if new_pos > pos[i] {
                (-1, pos[i] + 1, new_pos)
            } else {
                (1, new_pos, pos[i] - 1)
            };

            for j in 0..(length as usize) {
                if pos[j] >= lbound && pos[j] <= ubound {
                    pos[j] = (pos[j] + dir + length) % length;
                }
            }

            pos[i] = new_pos;
        }
    }

    let idx_0 = input
        .iter()
        .enumerate()
        .find(|(_, &p)| p == 0)
        .unwrap()
        .0;

    [1000, 2000, 3000]
        .into_iter()
        .map(|key| {
            let pair = pos
                .iter()
                .enumerate()
                .find(|(_, &value)| value == (key + pos[idx_0]) % length)
                .unwrap();
            input[pair.0]
        })
        .sum()
}
