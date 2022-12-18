// #![no_std]
use scapegoat::{SgMap, SgSet};
use tinyvec::ArrayVec;

const DIR: [[i8; 3]; 6] = [
    [0, 0, 1],
    [0, 0, -1],
    [0, 1, 0],
    [0, -1, 0],
    [1, 0, 0],
    [-1, 0, 0],
];

fn main() {
    let input = include_str!("../input.txt");

    let mut droplet: SgSet<[i8; 3], 3000> = SgSet::new();
    for line in input.lines() {
        let mut cube = [0i8; 3];
        line.split(',')
            .enumerate()
            .for_each(|(i, n)| cube[i] = n.parse::<i8>().unwrap());
        droplet.insert(cube);
    }

    let mut air: SgMap<[i8; 3], usize, 12000> = SgMap::new();
    let all: usize = droplet
        .iter()
        .map(|[x, y, z]| {
            DIR.iter()
                .filter(|[a, b, c]| {
                    let adj = [x + a, y + b, z + c];
                    let res = droplet.get(&adj).is_none();
                    if res {
                        *air.entry(adj).or_insert(0) += 1;
                    }
                    res
                })
                .count()
        })
        .sum();

    let inner: usize = air
        .into_iter()
        .filter(|(k, _)| {
            let mut stack: ArrayVec<[[i8;3]; 12000]> = ArrayVec::new();
            stack.push(*k);
            let mut seen: SgSet<[i8; 3], 12000> = SgSet::new();

            while let Some([x, y, z]) = stack.pop() {
                if !(1..22).contains(&x) || !(1..22).contains(&y) || !(1..22).contains(&z) {
                    return false;
                } else if droplet.contains(&[x, y, z]) || seen.contains(&[x, y, z]) {
                    continue;
                } else {
                    DIR.iter().for_each(|[a, b, c]| {
                        stack.push([x + a, y + b, z + c]);
                    });
                    seen.insert([x, y, z]);
                }
            }
            true
        })
        .map(|(_, v)| v)
        .sum();

    println!("part1: {}", all);
    println!("part2: {}", all - inner);
}
