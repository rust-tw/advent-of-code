trait Cyclable {
    fn run_noop(&mut self) {
        self.run_cycle();
    }

    fn run_addx(&mut self) {
        self.run_cycle();
        self.run_cycle();
    } 

    fn run_instruction(&mut self, instruction: String);

    fn run_cycle(&mut self);
}

struct CycleState {
    cycle: i32,
    register_x: i32,
    signal_sum: i32
}

impl CycleState {
    fn new() -> Self {
        Self{
            cycle: 0,
            register_x: 1,
            signal_sum: 0
        }
    }
}

impl Cyclable for CycleState {
    fn run_instruction(&mut self, instruction: String) {
        let mut instruction_segments = instruction.split(" ");

        match instruction_segments.next().unwrap() {
            "noop" => self.run_noop(),
            "addx" => {
                self.run_addx();
                self.register_x += instruction_segments.next()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
            }
            _ => {}
        }
    }

    fn run_cycle(&mut self) {
        self.cycle += 1;
        if self.cycle == 20 || (self.cycle - 20) % 40 == 0 {
            self.signal_sum += self.cycle * self.register_x;
        }
    }
}

struct CrtState {
    cycle: i32,
    register_x: i32,
    crt_image: Vec<char>
}

impl CrtState {
    fn new() -> Self {
        Self{
            cycle: 0,
            register_x: 1,
            crt_image: vec![]
        }
    }
}

impl Cyclable for CrtState {
    fn run_instruction(&mut self, instruction: String) {
        let mut instruction_segments = instruction.split(" ");

        match instruction_segments.next().unwrap() {
            "noop" => self.run_noop(),
            "addx" => {
                self.run_addx();
                self.register_x += instruction_segments.next()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
            }
            _ => {}
        }
    }

    fn run_cycle(&mut self) {
        self.crt_image.push(if self.cycle >= self.register_x - 1 && self.cycle <= self.register_x + 1 {
            '.'
        } else {
            '#'
        });
        self.cycle += 1;
        if self.cycle % 40 == 0 {
            self.cycle = 0;
            self.crt_image.push('\n');
        }
    }
}

fn main() {
    let data = include_str!("../Day10.txt");
    let processed_data = process_data(data);

    println!("{}", part1(&processed_data));
    println!("{}", part2(&processed_data));
}

fn part1(data: &Vec<String>) -> i32 {
    data.iter()
        .fold(CycleState::new(), |mut state, instruction| {
            state.run_instruction(instruction.to_string());
            state
        }).signal_sum
}

fn part2(data: &Vec<String>) -> String {
    data.iter()
        .fold(CrtState::new(), |mut state, instruction| {
            state.run_instruction(instruction.to_string());
            state
        }).crt_image.iter().collect()
}

fn process_data(data: &'static str) -> Vec<String> {
    data.replace("\r\n", "\n")
        .split("\n")
        .map(|s| s.to_string())
        .collect()
}
