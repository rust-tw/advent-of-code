use tony84727::{parse_instruction, Instruction};

fn main() {
    let mut x = 1;
    let mut answer = 0;
    let mut next = 20;
    let mut clock = 0;
    for instruction in std::io::stdin()
        .lines()
        .map(|line| parse_instruction(&line.unwrap()))
    {
        match instruction {
            Instruction::Noop => {
                if clock == next {
                    answer += next * x;
                    next += 40;
                }
                clock += 1;
            }
            Instruction::Addx(delta) => {
                if next - clock <= 2 {
                    answer += next * x;
                    next += 40;
                }
                clock += 2;
                x += delta;
            }
        }
    }
    println!("{}", answer)
}
