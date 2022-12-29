use std::{error::Error, str::FromStr};

#[derive(Debug)]
pub struct Blueprint {
    pub recipes: [[u64; 4]; 4],
}

#[derive(Debug)]
pub enum ParseBlueprintError {
    InvalidFormat,
}

impl<T> From<T> for ParseBlueprintError
where
    T: Error,
{
    fn from(_value: T) -> Self {
        ParseBlueprintError::InvalidFormat
    }
}

impl FromStr for Blueprint {
    type Err = ParseBlueprintError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .split(' ')
            .filter(|&token| token.chars().all(|c| c.is_numeric()))
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        Ok(Blueprint {
            recipes: [
                [numbers[0], 0, 0, 0],          // ore
                [numbers[1], 0, 0, 0],          // clay
                [numbers[2], numbers[3], 0, 0], // obsidian
                [numbers[4], 0, numbers[5], 0], // geode
            ],
        })
    }
}
