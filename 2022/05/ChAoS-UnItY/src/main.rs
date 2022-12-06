extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use regex::Regex;

#[derive(Debug)]
struct Instruction {
    count: i32,
    from: i32,
    to: i32,
}

fn main() {
    let data = include_str!("../Day05.txt");
    let (stacks, instructions) = process_data(data);

    println!(
        "{:?}",
        part1(Rc::new(RefCell::new(stacks.clone())), &instructions)
    );
    println!(
        "{:?}",
        part2(Rc::new(RefCell::new(stacks.clone())), &instructions)
    );
}

fn part1(stacks: Rc<RefCell<Vec<VecDeque<char>>>>, instructions: &Vec<Instruction>) -> String {
    for Instruction { count, from, to } in instructions {
        for _ in 0..*count {
            let value = stacks.borrow_mut()[*from as usize].pop_front().unwrap();

            stacks.borrow_mut()[*to as usize].push_front(value);
        }
    }

    stacks.take().iter().map(|s| s[0]).collect::<String>()
}

fn part2(stacks: Rc<RefCell<Vec<VecDeque<char>>>>, instructions: &Vec<Instruction>) -> String {
    for Instruction { count, from, to } in instructions {
        let mut holder = VecDeque::new();

        for _ in 0..*count {
            holder.push_back(stacks.borrow_mut()[*from as usize].pop_front().unwrap());
        }

        holder
            .iter()
            .rev()
            .for_each(|elem| stacks.borrow_mut()[*to as usize].push_front(*elem));
    }

    stacks.take().iter().map(|s| s[0]).collect::<String>()
}

fn process_data(data: &'static str) -> (Vec<VecDeque<char>>, Vec<Instruction>) {
    data.replace("\r\n", "\n")
        .split_once("\n\n")
        .map(|(stack_graph, instructions)| {
            (
                process_stack_graph(stack_graph.to_string()),
                process_instructions(instructions.to_string()),
            )
        })
        .unwrap()
}

fn process_stack_graph(data: String) -> Vec<VecDeque<char>> {
    let mut splitted_string = data.split("\n").collect::<Vec<_>>();
    let mut stacks = vec![VecDeque::new(); (&splitted_string[0].len() + 1) / 4];

    for layer in splitted_string
        .drain(..splitted_string.len() - 1)
        .map(|line| {
            line.chars()
                .collect::<Vec<_>>()
                .chunks(4)
                .enumerate()
                .filter(|(_, elem)| !elem.iter().all(|c| c.is_whitespace()))
                .map(|(i, elem)| (i, elem[1]))
                .collect::<Vec<_>>()
        })
    {
        for (i, elem) in layer {
            stacks[i].push_back(elem);
        }
    }

    return stacks;
}

fn process_instructions(data: String) -> Vec<Instruction> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }

    data.split("\n")
        .map(|line| {
            let imms = RE
                .find_iter(line)
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            Instruction {
                count: imms[0],
                from: imms[1] - 1,
                to: imms[2] - 1,
            }
        })
        .collect()
}
