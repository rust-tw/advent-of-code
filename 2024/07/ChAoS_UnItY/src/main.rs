use itertools::Itertools;

fn main() {
    let content = include_str!("../Day07.txt");
    part1(content);
    part2(content);
}

fn concat(lhs: u64, rhs: u64) -> u64 {
    lhs * 10u64.pow(rhs.ilog10() + 1) + rhs
}

fn part1(content: &str) {
    let mut answer = 0;
    let cases = content
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(target, compounds)| {
            (
                target.parse::<u64>().unwrap(),
                compounds
                    .split(" ")
                    .map(|compound| compound.parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        });

    fn generate_combinations(length: usize) -> impl Iterator<Item = Vec<fn(u64, u64) -> u64>> {
        std::iter::repeat([u64::wrapping_add, u64::wrapping_mul])
            .take(length)
            .multi_cartesian_product()
    }

    for (target, compounds) in cases {
        let combinations = generate_combinations(compounds.len() - 1);

        for combination in combinations {
            let result = compounds
                .iter()
                .skip(1)
                .zip(combination)
                .fold(compounds[0], |acc, (&compound, op)| op(acc, compound));

            if result == target {
                answer += result;
                break;
            }
        }
    }

    println!("Part 1: {answer}");
}

fn part2(content: &str) {
    let mut answer = 0;
    let cases = content
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(target, compounds)| {
            (
                target.parse::<u64>().unwrap(),
                compounds
                    .split(" ")
                    .map(|compound| compound.parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        });

    fn generate_combinations(length: usize) -> impl Iterator<Item = Vec<fn(u64, u64) -> u64>> {
        std::iter::repeat([u64::wrapping_add, u64::wrapping_mul, concat])
            .take(length)
            .multi_cartesian_product()
    }

    for (target, compounds) in cases {
        let combinations = generate_combinations(compounds.len() - 1);

        for combination in combinations {
            let result = compounds
                .iter()
                .skip(1)
                .zip(combination)
                .fold(compounds[0], |acc, (&compound, op)| op(acc, compound));

            if result == target {
                answer += result;
                break;
            }
        }
    }

    println!("Part 2: {answer}");
}
