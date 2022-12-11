use std::collections::HashMap;

use lazy_static::lazy_static;
use tony84727::{parse_monkeys, Arithmetic};

const PRIMES: [i32; 9] = [2, 3, 5, 7, 11, 13, 17, 19, 23];

lazy_static! {
    static ref PRIMES_LOOKUP: HashMap<i32, usize> = {
        let mut table = HashMap::new();
        for (i, &p) in PRIMES.iter().enumerate() {
            table.insert(p, i);
        }
        table
    };
}

struct WorryLevel(Vec<i32>);

impl WorryLevel {
    fn new(x: i32) -> Self {
        let mut level = Self(vec![x; PRIMES.len()]);
        level.minify();
        level
    }
    fn add(&mut self, x: i32) {
        for residual in self.0.iter_mut() {
            *residual += x;
        }
        self.minify();
    }

    fn multiply(&mut self, x: i32) {
        for residual in self.0.iter_mut() {
            *residual *= x;
        }
        self.minify();
    }

    fn minify(&mut self) {
        for (i, residual) in self.0.iter_mut().enumerate() {
            *residual %= PRIMES[i];
        }
    }

    fn square(&mut self) {
        for residual in self.0.iter_mut() {
            *residual *= *residual;
        }
        self.minify();
    }

    fn test(&self, prime: i32) -> bool {
        self.0[*PRIMES_LOOKUP.get(&prime).unwrap()] == 0
    }
}

fn main() {
    let monkeys = parse_monkeys(std::io::stdin().lines().map(|line| line.unwrap()));
    let mut items = monkeys
        .iter()
        .map(|m| m.items.iter().map(|x| WorryLevel::new(*x)).collect())
        .collect::<Vec<Vec<WorryLevel>>>();
    let mut monkey_inspect = vec![0; monkeys.len()];
    for _round in 0..10000 {
        for (m, monkey) in monkeys.iter().enumerate() {
            let monkey_items = std::mem::replace(&mut items[m], vec![]);
            monkey_inspect[m] += monkey_items.len();
            for mut item in monkey_items.into_iter() {
                match monkey.operation {
                    Arithmetic::Square => {
                        item.square();
                    }
                    Arithmetic::Add(operand) => {
                        item.add(operand);
                    }
                    Arithmetic::Multiply(operand) => {
                        item.multiply(operand);
                    }
                }
                if item.test(monkey.test) {
                    items[monkey.true_to as usize].push(item);
                } else {
                    items[monkey.false_to as usize].push(item);
                }
            }
        }
    }
    monkey_inspect.sort_unstable();

    println!(
        "{}",
        monkey_inspect
            .into_iter()
            .rev()
            .take(2)
            .fold(1, |a, b| a * b)
    );
}

#[cfg(test)]
mod tests {
    use crate::WorryLevel;

    #[test]
    fn test_worry_level_add() {
        let mut level = WorryLevel::new(5);
        assert!(level.test(5));
        level.add(2);
        assert!(level.test(7));
        assert!(!level.test(5));
    }
}
