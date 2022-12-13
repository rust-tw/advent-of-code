//use std::env;
use std::fs;

fn main() {
    let file_path = "test.input";
    let contents = fs::read_to_string(file_path)
        .expect("Read file error");
    println!("{}", part1(contents.clone()));
    println!("{}", part2(contents.clone()));
}


fn part1(contents: String) -> i32 {
    let mut ans = 0;
    for ln in contents.lines() {
        let item = ln.split(',').collect::<Vec<&str>>();
        let item1 = item[0].split('-').collect::<Vec<&str>>();
        let item2 = item[1].split('-').collect::<Vec<&str>>();
        if (item1[0].parse::<i32>().unwrap() <= item2[0].parse::<i32>().unwrap() &&
            item1[1].parse::<i32>().unwrap() >= item2[1].parse::<i32>().unwrap() ) ||
           (item1[0].parse::<i32>().unwrap() >= item2[0].parse::<i32>().unwrap() &&
            item1[1].parse::<i32>().unwrap() <= item2[1].parse::<i32>().unwrap() ){
            ans += 1;
        }
    }
    ans
}

fn part2(contents: String) -> i32 {
    let mut ans = 0;
    for ln in contents.lines() {
        let item = ln.split(',').collect::<Vec<&str>>();
        let item1 = item[0].split('-').collect::<Vec<&str>>();
        let item2 = item[1].split('-').collect::<Vec<&str>>();
        if (item1[0].parse::<i32>().unwrap() <= item2[0].parse::<i32>().unwrap() &&
            item1[1].parse::<i32>().unwrap() >= item2[0].parse::<i32>().unwrap()) ||
           (item1[0].parse::<i32>().unwrap() <= item2[1].parse::<i32>().unwrap() &&
            item1[1].parse::<i32>().unwrap() >= item2[1].parse::<i32>().unwrap()) ||
           (item2[0].parse::<i32>().unwrap() <= item1[0].parse::<i32>().unwrap() &&
            item2[1].parse::<i32>().unwrap() >= item1[0].parse::<i32>().unwrap()) ||
           (item2[0].parse::<i32>().unwrap() <= item1[1].parse::<i32>().unwrap() &&
            item2[1].parse::<i32>().unwrap() >= item1[1].parse::<i32>().unwrap()) {
            ans += 1;
        }
    }
    ans
}
