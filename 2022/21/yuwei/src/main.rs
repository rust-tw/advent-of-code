use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
enum Job {
    Num(isize),
    Add(String, String),
    Mul(String, String),
    Sub(String, String),
    Div(String, String),
    Unknown,
}

impl Job {
    fn into_sub(self) -> Job {
        match self {
            Job::Add(x, y) => Job::Sub(x, y),
            Job::Mul(x, y) => Job::Sub(x, y),
            Job::Sub(x, y) => Job::Sub(x, y),
            Job::Div(x, y) => Job::Sub(x, y),
            _ => unreachable!(),
        }
    }

    fn get_arms(&self) -> Option<(&str, &str)> {
        match self {
            Job::Add(x, y) => Some((x, y)),
            Job::Mul(x, y) => Some((x, y)),
            Job::Sub(x, y) => Some((x, y)),
            Job::Div(x, y) => Some((x, y)),
            _ => None,
        }
    }
}

fn search(
    monkies: &HashMap<String, Job>,
    cache: &mut HashMap<String, isize>,
    name: &str,
) -> Option<isize> {
    if let Some(&val) = cache.get(name) {
        return Some(val);
    }
    let val = match &monkies[name] {
        Job::Num(n) => *n,
        Job::Add(x, y) => search(monkies, cache, x)? + search(monkies, cache, y)?,
        Job::Mul(x, y) => search(monkies, cache, x)? * search(monkies, cache, y)?,
        Job::Sub(x, y) => search(monkies, cache, x)? - search(monkies, cache, y)?,
        Job::Div(x, y) => search(monkies, cache, x)? / search(monkies, cache, y)?,
        _ => return None,
    };
    cache.insert(name.to_string(), val);
    Some(val)
}

fn main() {
    let input = include_str!("../input.txt");

    let mut monkies = input
        .lines()
        .map(|l| {
            let re = Regex::new(r"(\w+): (.*)").unwrap();
            let caps = re.captures(l).unwrap();
            let monkey = caps[1].to_string();
            let job = if !caps[2].contains(' ') {
                Job::Num(caps[2].parse().unwrap())
            } else {
                let (x, op, y) = caps[2].split(' ').collect_tuple().unwrap();
                match op {
                    "+" => Job::Add(x.to_string(), y.to_string()),
                    "*" => Job::Mul(x.to_string(), y.to_string()),
                    "-" => Job::Sub(x.to_string(), y.to_string()),
                    "/" => Job::Div(x.to_string(), y.to_string()),
                    _ => panic!("Unknown op: {}", op),
                }
            };
            (monkey, job)
        })
        .collect::<HashMap<_, _>>();

    let mut cache = HashMap::new();
    let part1 = search(&monkies, &mut cache, "root");
    println!("Part 1: {:?}", part1);

    monkies.insert("humn".to_string(), Job::Unknown);
    let root_job = monkies.remove("root").unwrap();
    monkies.insert("root".to_string(), root_job.into_sub());
    let mut cache = HashMap::new();

    let mut part2 = 0;
    let mut job = &monkies["root"];
    while let Some((left, right)) = job.get_arms() {
        match (
            search(&monkies, &mut cache, left),
            search(&monkies, &mut cache, right),
        ) {
            (Some(n), None) => {
                match job {
                    Job::Add(_, _) => part2 -= n,
                    Job::Mul(_, _) => part2 /= n,
                    Job::Sub(_, _) => part2 = n - part2,
                    Job::Div(_, _) => part2 = n / part2,
                    _ => panic!("Root should have a operator."),
                }
                job = &monkies[right];
            }
            (None, Some(n)) => {
                match job {
                    Job::Add(_, _) => part2 -= n,
                    Job::Mul(_, _) => part2 /= n,
                    Job::Sub(_, _) => part2 += n,
                    Job::Div(_, _) => part2 *= n,
                    _ => panic!("Root should have a operator."),
                }
                job = &monkies[left];
            }
            _ => unreachable!(),
        };
    }

    println!("{part2} {job:?}");
}
