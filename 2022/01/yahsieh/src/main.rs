#![feature(iter_partition_in_place)]

fn parse(input: &str) -> Vec<i32> {
    input
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .fold(0, |acc, i| acc + i.parse::<i32>().unwrap())
        })
        .collect()
}

fn q1(input: &str) -> i32 {
    parse(input).into_iter().max().unwrap_or(0)
}

fn q2(input: &str) -> i32 {
    let mut inp = parse(input);
    let k = inp.len() - 3 - 1;
    qselect(&mut inp, k)
}

fn qselect(v: &mut [i32], k: usize) -> i32 {
    let mut l: usize = 0;
    let mut r: usize = v.len() - 1;
    while l < r {
        let pivot = v[r].clone();
        let nl = v[l..r].iter_mut().partition_in_place(|&n| n <= pivot);
        (v[l + nl], v[r]) = (v[r], v[l + nl]);
        if nl < k {
            l = l + nl + 1;
        } else if nl > k {
            r = l + nl - 1;
        } else {
            break;
        }
    }
    v.iter().rev().take(3).sum()
}

fn main() {
    let input = include_str!("../test");
    println!("{}", q1(input));
    println!("{}", q2(input));
}
