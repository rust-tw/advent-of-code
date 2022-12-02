fn convert_elf(s: &str) -> i32 {
    match s {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => unreachable!(),
    }
}
fn convert_my(s: &str) -> i32 {
    match s {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => unreachable!(),
    }
}

/// Return the judge result
fn judge(elf: i32, my: i32) -> i32 {
    static JUDGE_TABLE: &'static [&'static [i32]] = &[&[3, 6, 0], &[0, 3, 6], &[6, 0, 3]];
    // 1 for Rock
    // 2 for Paper
    // 3 for Scissor

    // elf, my
    // 1, 1 -> 3
    // 1, 2 -> 6
    // 1, 3 -> 0
    //
    // 2, 1 -> 0
    // 2, 2 -> 3
    // 2, 3 -> 6
    //
    // 3, 1 -> 6
    // 3, 2 -> 0
    // 3, 3 -> 3

    my + JUDGE_TABLE[(elf - 1) as usize][(my - 1) as usize]
}

fn convert_instruction(s: &str) -> i32 {
    match s {
        "X" => 0, // we will lose
        "Y" => 3, // we will draw
        "Z" => 6, // we will win
        _ => unreachable!(),
    }
}

fn judge_with_instruction(elf: i32, instr: i32) -> i32 {
    static JUDGE_INSTR_TABLE: &'static [&'static [i32]] = &[&[3, 1, 2], &[1, 2, 3], &[2, 3, 1]];
    // elf, instr -> our move
    // 1, 0 -> 3
    // 1, 3 -> 1
    // 1, 6 -> 2
    //
    // 2, 0 -> 1
    // 2, 3 -> 2
    // 2, 6 -> 3
    //
    // 3, 0 -> 2
    // 3, 3 -> 3
    // 3, 6 -> 1
    JUDGE_INSTR_TABLE[(elf - 1) as usize][(instr / 3) as usize] + instr
}

fn main() {
    let case1: i32 = include_str!("../input.txt")
        .lines()
        .map(|x| {
            let turn: Vec<&str> = x.split(" ").collect();
            let elf = convert_elf(turn[0]);
            let my = convert_my(turn[1]);
            judge(elf, my)
        })
        .sum();
    println!("case1: {}", case1);

    let case2: i32 = include_str!("../input.txt")
        .lines()
        .map(|x| {
            let turn: Vec<&str> = x.split(" ").collect();
            let elf = convert_elf(turn[0]);
            let instruction = convert_instruction(turn[1]);
            judge_with_instruction(elf, instruction)
        })
        .sum();
    println!("case2: {}", case2);
}
