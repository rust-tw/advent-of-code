use std::collections::HashSet;

fn solve1(input: &str) -> i32 {
    input
        .lines()
        .map(|s| {
            let (win, case) = s.split_once(':').unwrap().1.split_once('|').unwrap();
            let win = win
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let case = case
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let map: HashSet<i32> = HashSet::from_iter(win);

            let count = case.iter().filter(|x| map.contains(x)).count() as i32;
            if count == 0 {
                0
            } else {
                2_i32.pow(std::cmp::max(0, count - 1) as u32)
            }
        })
        .sum()
}

fn solve2(input: &str) -> i32 {
    let vstr = input.lines().collect::<Vec<&str>>();
    let mut v = vec![1; vstr.len()];
    vstr.iter().for_each(|s| {
        let (id_str, card_str) = s.split_once(':').unwrap();
        let id = id_str
            .split_once(' ')
            .unwrap()
            .1
            .trim()
            .parse::<usize>()
            .unwrap()
            - 1;
        let (win, case) = card_str.split_once('|').unwrap();
        let win = win
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let case = case
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let map: HashSet<i32> = HashSet::from_iter(win);

        let count = case.iter().filter(|x| map.contains(x)).count();
        if count > 0 {
            let max = std::cmp::min(vstr.len() - 1, id + count);
            for i in (id + 1)..=max {
                v[i] += v[id];
            }
        }
    });
    v.iter().sum()
}

fn main() {
    let my_str = include_str!("input");
    println!("{}", solve1(my_str));
    println!("{}", solve2(my_str));
}
