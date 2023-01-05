use core::panic;
use std::{fmt::Display, str::FromStr};

pub struct Snafu {
    data: i64,
}

impl Snafu {
    pub fn new(data: i64) -> Self {
        Self { data }
    }
}

impl From<i64> for Snafu {
    fn from(value: i64) -> Self {
        Self { data: value }
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = if self.data == 0 {
            String::from("0")
        } else {
            let mut v = vec![];
            let mut num = self.data;
            while num != 0 {
                match (num + 2) % 5 {
                    0 => v.push('='),
                    1 => v.push('-'),
                    2 => v.push('0'),
                    3 => v.push('1'),
                    4 => v.push('2'),
                    _ => panic!(""),
                };
                num = (num + 2) / 5;
            }

            v.into_iter().rev().collect()
        };

        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub struct ParseSnafuError;

impl FromStr for Snafu {
    type Err = ParseSnafuError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = 0;
        for c in s.chars() {
            let digit = match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => return Err(ParseSnafuError)
            };
            data *= 5;
            data += digit;
        }
        Ok(Snafu { data })
    }
}

impl From<Snafu> for i64 {
    fn from(value: Snafu) -> Self {
        value.data
    }
}
