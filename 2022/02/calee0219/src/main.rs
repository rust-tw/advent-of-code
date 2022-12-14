use std::fs;
use std::collections::HashMap;


fn main() {
    let file_path = "test.input";
    let contents = fs::read_to_string(file_path)
        .expect("File Read error");

    println!("{}", part1(contents.clone()));
    println!("{}", part2(contents.clone()));
}

fn part1(contents: String) -> i32 {
    let score: HashMap<String, i32> = HashMap::from(
        [
            ("A".to_string(), 1),
            ("B".to_string(), 2),
            ("C".to_string(), 3),
            ("X".to_string(), 1),
            ("Y".to_string(), 2),
            ("Z".to_string(), 3),
        ]
    );
    let mut ans: i32 = 0;
    for ln in contents.lines() {
        let token: Vec<i32> = ln.split_whitespace().map(|x| score[x]).collect();
        ans += token[1];
        match ((token[1] - 1) - (token[0] - 1)).rem_euclid(3) {
            0 => ans += 3,
            1 => ans += 6,
            //2 => ans += 0,
            _ => (),
        }
    }
    ans
}

fn part2(contents: String) -> i32 {
    let score: HashMap<String, i32> = HashMap::from(
        [
            ("A".to_string(), 1),
            ("B".to_string(), 2),
            ("C".to_string(), 3),
            ("X".to_string(), 0),
            ("Y".to_string(), 3),
            ("Z".to_string(), 6),
        ]
    );
    let mut ans: i32 = 0;
    for ln in contents.lines() {
        let token: Vec<i32> = ln.split_whitespace().map(|x| score[x]).collect();
        ans += token[1];
        match token[1] {
            0 => ans += (token[0] - 2).rem_euclid(3) + 1 ,
            3 => ans += token[0],
            6 => ans += token[0] % 3 + 1,
            _ => (),
        }
    }
    ans
}
