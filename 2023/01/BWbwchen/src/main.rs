fn solve1(input: &str) -> u32 {
    input.lines().fold(0, |acc, x| {
        let mut first_digit = 0;
        let mut second_digit = 0;
        for c in x.chars() {
            if c.is_ascii_digit() {
                first_digit = c.to_digit(10).unwrap();
                break;
            }
        }
        for c in x.chars().rev() {
            if c.is_ascii_digit() {
                second_digit = c.to_digit(10).unwrap();
                break;
            }
        }

        acc + first_digit * 10 + second_digit
    })
}

use once_cell::sync::Lazy;

static NUM_STR: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
});

enum Dir {
    Forward,
    Backward,
}

fn try_get_num(input: &str, direction: Dir) -> Option<u32> {
    match direction {
        Dir::Forward => {
            let first_char = input.chars().next().unwrap();
            if first_char.is_ascii_digit() {
                return first_char.to_digit(10);
            }
            for (i, &ns) in NUM_STR.iter().enumerate() {
                if input.starts_with(ns) {
                    return Some(i as u32);
                }
            }
        }
        Dir::Backward => {
            let last_char = input.chars().last().unwrap();
            if last_char.is_ascii_digit() {
                return last_char.to_digit(10);
            }

            for (i, &ns) in NUM_STR.iter().enumerate() {
                if input.ends_with(ns) {
                    return Some(i as u32);
                }
            }
        }
    }
    None
}

fn solve2(input: &str) -> u32 {
    input.lines().fold(0, |acc, x| {
        let mut first_digit = 0;
        let mut second_digit = 0;

        for i in 0..x.len() {
            if let Some(d) = try_get_num(&x[i..], Dir::Forward) {
                first_digit = d;
                break;
            };
        }
        for i in (0..x.len()).rev() {
            if let Some(d) = try_get_num(&x[..i + 1], Dir::Backward) {
                second_digit = d;
                break;
            };
        }

        acc + first_digit * 10 + second_digit
    })
}

fn main() {
    let my_str = include_str!("day01-input");
    println!("{}", solve1(my_str));
    println!("{}", solve2(my_str));
}
