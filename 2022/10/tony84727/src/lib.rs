pub enum Instruction {
    Addx(i32),
    Noop,
}

pub fn parse_instruction(instruction: &str) -> Instruction {
    let mut semgnets = instruction.trim().split(" ");
    match semgnets.next() {
        Some("noop") => Instruction::Noop,
        _ => Instruction::Addx(semgnets.next().unwrap().parse().unwrap()),
    }
}
