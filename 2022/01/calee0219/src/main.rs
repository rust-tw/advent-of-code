use std::fs;
use std::cmp;
use std::collections::BinaryHeap;

const HEAP_SIZE: usize = 3;

fn main() {
    let file_path = "test.input";
    let contents = fs::read_to_string(file_path)
        .expect("File Read error");
    println!("{}", part1(contents.clone()));
    println!("{}", part2(contents.clone()));
}

fn part1(contents: String) -> i32 {
    let mut max_val = 0;
    let mut curr = 0;
    for ln in contents.lines() {
        if ln == "" {
            max_val = cmp::max(max_val, curr);
            curr = 0;
        } else {
            curr += ln.parse::<i32>().unwrap();
        }
    }
    max_val = cmp::max(max_val, curr);
    max_val
}

fn part2(contents: String) -> i32 {
    let mut heap = BinaryHeap::with_capacity(HEAP_SIZE);
    let mut curr = 0;
    for ln in contents.lines() {
        if ln == "" {
            heap.push(-curr);
            if heap.len() > HEAP_SIZE {
                heap.pop();
            }
            curr = 0;
        } else {
            curr += ln.parse::<i32>().unwrap();
        }
    }
    -heap.iter().sum::<i32>()
}
