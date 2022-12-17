use lazy_static::lazy_static;
use ndarray::Array3;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ").unwrap();
}

struct Valve<'a> {
    valve: &'a str,
    rate: usize,
    tunnels: Vec<&'a str>,
}

fn main() {
    let input = include_str!("../input.txt");

    let mut valves = Vec::<Valve>::new();
    for line in input.trim().lines() {
        let capture = REGEX.captures(line).unwrap();
        let valve = capture.get(1).unwrap().as_str();
        let rate = capture[2].parse().unwrap();
        let tunnels = line[capture.get(0).unwrap().end()..].split(", ").collect();
        valves.push(Valve {
            valve,
            rate,
            tunnels,
        });
    }
    valves.sort_by(|a, b| b.rate.cmp(&a.rate));
    let index = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.valve, i))
        .collect::<HashMap<_, _>>();

    let n = valves.len();
    let mut rate = vec![0; n];
    let mut tunnels = vec![vec![0; 0]; n];
    for v in valves.iter() {
        let i = index[v.valve];
        rate[i] = v.rate;
        for j in v.tunnels.iter() {
            tunnels[i].push(index[j]);
        }
    }

    let count = valves.iter().filter(|v| v.rate > 0).count();
    let bitflag = 1 << count;
    // Dynamic programming [time left, current node, bitflag of valves]
    let mut array = Array3::<usize>::zeros([30, n, bitflag]);
    for t in 1..30 {
        for i in 0..n {
            let x = 1 << i;
            for flag in 0..bitflag {
                let mut res = array[(t, i, flag)];
                if x & flag != 0 && t >= 1 {
                    res = res.max(array[(t - 1, i, flag - x)] + rate[i] * t);
                }
                for j in tunnels[i].iter() {
                    res = res.max(array[(t - 1, *j, flag)]);
                }
                array[(t, i, flag)] = res;
            }
        }
    }

    let aa = index["AA"];
    let part1 = array[(29, aa, bitflag - 1)];
    let mut part2 = 0;
    for i in 0..bitflag / 2 {
        let j = bitflag - 1 - i;
        part2 = part2.max(array[(25, aa, i)] + array[(25, aa, j)]);
    }

    println!("{part1}");
    println!("{part2}");
}
