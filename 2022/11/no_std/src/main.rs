// This is no_std for sure. I just disable it to println.
// #![no_std]
use tinyvec::{ArrayVec, array_vec};

fn main() {
    let monkes = [
        Monke {
            items: array_vec![53, 89, 62, 57, 74, 51, 83, 97],
            operation: Op::Mul(3),
            div: 13,
            test: (1, 5),
            inspect: 0,
        },
        Monke {
            items: array_vec![85, 94, 97, 92, 56],
            operation: Op::Add(2),
            div: 19,
            test: (5, 2),
            inspect: 0,
        },
        Monke {
            items: array_vec![86, 82, 82],
            operation: Op::Add(1),
            div: 11,
            test: (3, 4),
            inspect: 0,
        },
        Monke {
            items: array_vec![94, 68],
            operation: Op::Add(5),
            div: 17,
            test: (7, 6),
            inspect: 0,
        },
        Monke {
            items: array_vec![83, 62, 74, 58, 96, 68, 85],
            operation: Op::Add(4),
            div: 3,
            test: (3, 6),
            inspect: 0,
        },
        Monke {
            items: array_vec![50, 68, 95, 82],
            operation: Op::Add(8),
            div: 7,
            test: (2, 4),
            inspect: 0,
        },
        Monke {
            items: array_vec![75],
            operation: Op::Mul(7),
            div: 5,
            test: (7, 0),
            inspect: 0,
        },
        Monke {
            items: array_vec![92, 52, 85, 89, 68, 82],
            operation: Op::MulSelf,
            div: 2,
            test: (0, 1),
            inspect: 0,
        },
    ];

    shenanigans(monkes.clone(), 20, 3);
    let modulus = monkes.iter().map(|monke| monke.div).product();
    shenanigans(monkes, 10000, modulus);
}

#[derive(Clone)]
enum Op {
    Add(usize),
    Mul(usize),
    MulSelf,
}

#[derive(Clone)]
struct Monke {
    items: ArrayVec<[usize; 64]>,
    operation: Op,
    div: usize,
    test: (usize, usize),
    inspect: usize,
}

impl Op {
    fn part1(&self, item: usize, modulus: usize) -> usize {
        match self {
            Op::Add(i) => (item + i) / modulus,
            Op::Mul(i) => (item * i) / modulus,
            Op::MulSelf => (item * item) / modulus,
        }
    }

    fn part2(&self, item: usize, modulus: usize) -> usize {
        match self {
            Op::Add(i) => (item + i) % modulus,
            Op::Mul(i) => (item * i) % modulus,
            Op::MulSelf => (item * item) % modulus,
        }
    }
}

fn shenanigans<const N: usize>(mut monkes: [Monke; N], rounds: usize, modulus: usize) {
    let len = monkes.len();
    let worry = if modulus == 3 { Op::part1 } else { Op::part2 };

    for _ in 0..rounds {
        for l in 0..len {
            let monke = &mut monkes[l];
            let inspect = monke.items.len();
            monke.inspect += inspect;

            for item in core::mem::take(&mut monke.items) {
                let monke = &monkes[l];
                let level = worry(&monke.operation, item, modulus);

                let next = if level % monke.div == 0 {
                    monke.test.0
                } else {
                    monke.test.1
                };

                monkes[next].items.push(level)
            }
        }
    }
    
    
    monkes.sort_by_key(|monke| monke.inspect);
    let business = monkes[len - 1].inspect * monkes[len - 2].inspect;
    println!("{business}");
}
