use std::{error::Error, str::FromStr};

pub fn challenge_10(lines: Vec<&str>) -> (i32, Vec<String>) {
    let mut history = vec![];
    let mut last_value = 1;
    lines
        .iter()
        .map(|line| line.parse::<Instuction>().unwrap())
        .for_each(|inst| match inst {
            Instuction::Addx(num) => {
                history.push(last_value);
                history.push(last_value);
                last_value = last_value + num;
            }
            Instuction::Noop => {
                history.push(last_value);
            }
        });

    println!("Length = {}", history.len());

    let signal_strength = [20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|n| n * history[(n - 1) as usize])
        .sum();

    let screen = history
        .into_iter()
        .enumerate()
        .map(|(idx, x)| {
            let pos = (idx % 40) as i32;
            if pos >= (x - 1) && pos <= (x + 1) {
                '#'
            } else {
                '.'
            }
        })
        .collect::<Vec<char>>()
        .chunks(40)
        .map(|chunk| chunk.iter().collect())
        .collect();

    (signal_strength, screen)
}

#[derive(Debug)]
enum Instuction {
    Addx(i32),
    Noop,
}

impl FromStr for Instuction {
    type Err = ParseInstructionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split(' ').collect::<Vec<_>>();
        let opcode = *(tokens.get(0).ok_or(Self::Err::InvalidFormat)?);
        match opcode {
            "addx" => {
                let param = tokens
                    .get(1)
                    .ok_or(Self::Err::InvalidFormat)?
                    .parse::<i32>()?;
                Ok(Self::Addx(param))
            }
            "noop" => Ok(Self::Noop),
            _ => Err(Self::Err::UnsupportedOpcode),
        }
    }
}

#[derive(Debug)]
enum ParseInstructionError {
    InvalidFormat,
    UnsupportedOpcode,
}

impl<T> From<T> for ParseInstructionError
where
    T: Error,
{
    fn from(_: T) -> Self {
        Self::InvalidFormat
    }
}
