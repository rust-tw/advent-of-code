use tony84727::{parse_instruction, Instruction};

fn render(x: i32, clock: i32) -> bool {
    let column = clock % 40;
    column >= x - 1 && column <= x + 1
}

fn draw(x: i32, clock: i32) {
    if clock % 40 == 0 {
        println!("");
    }
    print!("{}", if render(x, clock) { "#" } else { "." });
}

fn main() {
    let mut x = 1;
    let mut clock = 0;
    for instruction in std::io::stdin()
        .lines()
        .map(|line| parse_instruction(&line.unwrap()))
    {
        draw(x, clock);
        match instruction {
            Instruction::Addx(delta) => {
                clock += 1;
                draw(x, clock);
                x += delta;
            }
            Instruction::Noop => {}
        }
        clock += 1;
    }
}
