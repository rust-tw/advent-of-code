use std::fmt::Debug;

pub fn challenge_11(lines: Vec<&str>) -> (usize, usize) {
    let (owns, monkeys): (Vec<Vec<_>>, Vec<_>) = lines
        .split(|line| line.is_empty())
        .map(|group| Monkey::parse_lines(group))
        .unzip();

    let item_states = owns
        .iter()
        .enumerate()
        .flat_map(|(owner, worries)| worries.iter().map(move |&worry| ItemState { owner, worry }))
        .collect::<Vec<_>>();

    let part1 = get_answer(20, 3, 1, &monkeys, item_states.clone());

    // The modulo should be the Greatest Common Divider in fact.
    // But since all `test_div` are primes, we can simply product them up.
    let modulo = monkeys
        .iter()
        .map(|monkey| monkey.test_div)
        .product::<u64>();

    let part2 = get_answer(10000, 1, modulo, &monkeys, item_states);

    (part1, part2)
}

fn get_answer(
    loop_count: usize,
    divider: u64,
    modulo: u64,
    monkeys: &Vec<Monkey>,
    start_state: Vec<ItemState>,
) -> usize {
    let mut counts = vec![0; monkeys.len()];
    let _ = monkeys
        .iter()
        .enumerate()
        .cycle()
        .take(loop_count * monkeys.len())
        .fold(start_state, |old_state, (index, monkey)| {
            let (count, new_owns) = monkey.process(index, &old_state, divider, modulo);
            counts[index] += count;
            new_owns
        });

    counts.sort_unstable();

    counts.iter().rev().take(2).product()
}

#[derive(Debug, Clone, Copy)]
struct ItemState {
    owner: usize,
    worry: u64,
}

struct Monkey {
    operator: Box<dyn Fn(u64) -> u64>,
    test_div: u64,
    target_true: usize,
    target_false: usize,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Monkey (test_div: {}, target_true: {}, target_false: {})",
            self.test_div, self.target_true, self.target_false
        )
    }
}

impl Monkey {
    fn parse_lines(lines: &[&str]) -> (Vec<u64>, Self) {
        let v_lines = lines.into_iter().skip(1).collect::<Vec<_>>();
        let owns = v_lines[0]
            .trim()
            .split(": ")
            .skip(1)
            .next()
            .unwrap()
            .split(", ")
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let operation = v_lines[1]
            .trim()
            .split(' ')
            .skip_while(|s| *s != "=")
            .skip(2)
            .collect::<Vec<_>>();
        let operator: Box<dyn Fn(u64) -> u64> = match (operation[0], operation[1].parse::<u64>()) {
            ("+", Ok(num)) => Box::new(move |old| old + num),
            ("+", Err(_)) => Box::new(|old| old + old),
            ("*", Ok(num)) => Box::new(move |old| old * num),
            ("*", Err(_)) => Box::new(|old| old * old),
            _ => Box::new(|old| old),
        };

        let test_div = v_lines[2]
            .split(' ')
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let target_true = v_lines[3]
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let target_false = v_lines[4]
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        (
            owns,
            Self {
                operator,
                test_div,
                target_true,
                target_false,
            },
        )
    }

    fn process(
        &self,
        index: usize,
        owns: &Vec<ItemState>,
        divider: u64,
        modulo: u64,
    ) -> (usize, Vec<ItemState>) {
        let count = owns.iter().filter(|item| item.owner == index).count();
        let new_owns = owns
            .iter()
            .map(|&item| {
                if item.owner == index {
                    let new_level = (self.operator)(item.worry) / divider % modulo;
                    if new_level % self.test_div == 0 {
                        ItemState {
                            owner: self.target_true,
                            worry: new_level,
                        }
                    } else {
                        ItemState {
                            owner: self.target_false,
                            worry: new_level,
                        }
                    }
                } else {
                    item
                }
            })
            .collect();

        (count, new_owns)
    }
}
