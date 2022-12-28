use std::collections::VecDeque;
use std::fmt::Display;
use std::str::FromStr;
use std::vec;

#[derive(Clone)]
pub struct Grove {
    body: VecDeque<VecDeque<u8>>,
}

#[derive(Debug)]
pub enum ParseGroveError {
    InvalidFormat,
}

impl FromStr for Grove {
    type Err = ParseGroveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Ok(0),
                        '#' => Ok(1),
                        _ => Err(ParseGroveError::InvalidFormat),
                    })
                    .collect::<Result<VecDeque<_>, _>>()
            })
            .collect::<Result<VecDeque<_>, _>>()
            .map(|mut body| {
                body.iter_mut().for_each(|row| {
                    row.push_back(0);
                    row.push_front(0);
                });
                let width = body[0].len();
                body.push_back(vec![0; width].into());
                body.push_front(vec![0; width].into());

                Grove { body }
            })
    }
}

impl Display for Grove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .body
            .iter()
            .map(|row| {
                row.iter()
                    .map(|v| if *v == 0 { '.' } else { '#' })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", s)
    }
}

impl Grove {
    pub fn width(&self) -> i32 {
        self.body[0].len() as i32
    }

    pub fn height(&self) -> i32 {
        self.body.len() as i32
    }

    pub fn get(&self, x: i32, y: i32) -> u8 {
        if y >= 0 && (y as usize) < self.body.len() {
            if x >= 0 && (x as usize) < self.body[y as usize].len() {
                return self.body[y as usize][x as usize];
            }
        }
        return 0;
    }

    pub fn set(&mut self, x: i32, y: i32, value: u8) {
        if y >= 0 && (y as usize) < self.body.len() {
            if x >= 0 && (x as usize) < self.body[y as usize].len() {
                self.body[y as usize][x as usize] = value;
            }
        }
    }

    pub fn check_and_grow(&mut self) {
        if self.body.iter().any(|row| row[0] == 1) {
            self.body.iter_mut().for_each(|row| row.push_front(0));
        }
        if self.body.iter().any(|row| row[row.len() - 1] == 1) {
            self.body.iter_mut().for_each(|row| row.push_back(0));
        }
        let width = self.body[0].len();
        if self.body[0].iter().any(|c| *c == 1) {
            self.body.push_front(vec![0; width].into())
        }
        if self.body[self.body.len() - 1].iter().any(|c| *c == 1) {
            self.body.push_back(vec![0; width].into())
        }
    }
}
