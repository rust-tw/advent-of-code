use std::fs;
use std::collections::HashSet;

fn main() {
    let file_path = "test.input";
    let contents = fs::read_to_string(file_path)
        .expect("File read error");

    println!("{}", part1(contents.clone()));
    println!("{}", part2(contents.clone()));
}

fn char2num(c: char) -> u8 {
    let n = c as u8;
    //println!("{} {} {}", c, n, n & 31);
    if n < 96 {
        (n & 31) + 26
    } else {
        n & 31
    }
}

fn part1(contents: String) -> i32 {
    let mut ans: i32 = 0;
    for ln in contents.lines() {
        let n = ln.len();
        let mut set = HashSet::new();
        for (idx, ch) in ln.chars().enumerate() {
            if idx < n/2 {
                set.insert(ch);
            } else {
                if set.contains(&ch) {
                    ans += char2num(ch) as i32;
                    //println!("{}", ans);
                    break;
                }
            }
        }
    }
    ans
}

fn part2(contents: String) -> i32 {
    let mut ans: i32 = 0;
    let mut set: HashSet<char> = HashSet::new();
    for (idx, ln) in contents.lines().enumerate() {
        let mut tmp = HashSet::new();
        if idx % 3 == 0 {
            for ch in ln.chars() {
                tmp.insert(ch);
            }
            set = tmp;
        } else {
            for ch in ln.chars() {
                if set.contains(&ch) {
                    tmp.insert(ch);
                }
            }
            set = tmp;
        }
        if idx % 3 == 2 {
            ans += char2num(*set.iter().next().unwrap()) as i32;
        }
    }
    ans
}
