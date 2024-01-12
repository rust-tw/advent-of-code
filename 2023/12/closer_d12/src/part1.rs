pub fn solve(input: &Vec<&str>) -> u64 {
    input
        .iter()
        .map(|line| {
            let mut parts = line.split(' ');
            let springs = parts
                .next()
                .unwrap()
                .chars()
                // For each springs list, attach a '.' at tail. This make the `can_fit()`
                // algorithm more generic.
                .chain(['.'].into_iter())
                .collect::<Vec<_>>();
            let sequences = parts
                .next()
                .unwrap()
                .split(',')
                .map(|token| token.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            solve_line(&springs, &sequences) as u64
        })
        .sum()
}

pub fn solve_line(springs: &[char], sequences: &[usize]) -> u64 {
    if sequences.is_empty() {
        if springs.contains(&'#') {
            0
        } else {
            1
        }
    } else {
        let end = springs
            .iter()
            .enumerate()
            .find(|(_, c)| **c == '#')
            .unwrap_or((springs.len(), &'_'))
            .0;
        (0..=end)
            .filter(|idx| closer_d12::can_fit(&springs[*idx..], sequences[0]))
            .map(|idx| solve_line(&springs[(idx + sequences[0] + 1)..], &sequences[1..]))
            .sum()
    }
}
