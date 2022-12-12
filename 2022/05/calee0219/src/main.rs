use std::fs;
use std::vec::Vec;

fn main() {
    let file_path_stack = "test_stack.input";
    let file_path_ops = "test_ops.input";
    let stack_contents = fs::read_to_string(file_path_stack)
        .expect("Read file error");
    let ops_contents = fs::read_to_string(file_path_ops)
        .expect("Read file error");
    println!("{}", part1(stack_contents.clone(), ops_contents.clone()));
    println!("{}", part2(stack_contents.clone(), ops_contents.clone()));
}

fn part1(stack_contents: String, ops_contents: String) -> String {
    // Read stack
    let mut stack = Vec::new();
    for ln in stack_contents.lines() {
        stack.push(ln.split(" ").collect::<Vec<&str>>());
    }
    // Do operation
    for ln in ops_contents.lines() {
        let tmp = ln.split(" ").collect::<Vec<&str>>();
        let mut no: i32 = tmp[1].parse().unwrap();
        let from: usize = tmp[3].parse::<usize>().unwrap()-1;
        let to: usize = tmp[5].parse::<usize>().unwrap()-1;

        while no > 0 {
            let tmp = stack[from].pop().unwrap();
            stack[to].push(tmp);
            no -= 1;
        }
    }

    let mut ret: String = "".to_string();
    for it in stack {
        ret += it.last().unwrap();
    }
    ret
}

fn part2(stack_contents: String, ops_contents: String) -> String {
    // Read stack
    let mut stack = Vec::new();
    for ln in stack_contents.lines() {
        stack.push(ln.split(" ").collect::<Vec<&str>>());
    }
    // Do operation
    for ln in ops_contents.lines() {
        let tmp = ln.split(" ").collect::<Vec<&str>>();
        let mut no: i32 = tmp[1].parse().unwrap();
        let from: usize = tmp[3].parse::<usize>().unwrap()-1;
        let to: usize = tmp[5].parse::<usize>().unwrap()-1;

        let mut tmp_stk = Vec::new();
        while no > 0 {
            tmp_stk.push(stack[from].pop().unwrap());
            no -= 1;
        }
        while !tmp_stk.is_empty() {
            stack[to].push(tmp_stk.pop().unwrap());
        }
    }

    let mut ret: String = "".to_string();
    for it in stack {
        ret += it.last().unwrap();
    }
    ret
}
