use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Beacon {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Sensor {
    x: i32,
    y: i32,
    beacon: Beacon,
    range: i32,
}

impl Sensor {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Sensor {
            x: 0,
            y: 0,
            beacon: Beacon { x: 0, y: 0 },
            range: 0,
        }
    }

    pub fn get_beacon(&self) -> Beacon {
        self.beacon
    }

    #[allow(dead_code)]
    pub fn scanned_blocks(&self, y: i32) -> Vec<i32> {
        let span = self.range - (self.y.abs_diff(y) as i32);
        ((self.x - span)..=(self.x + span)).collect()
    }

    pub fn range_in_row(&self, y: i32) -> Option<(i32, i32)> {
        let span = self.range - (self.y.abs_diff(y) as i32);
        if span < 0 {
            None
        } else {
            Some((self.x - span, self.x + span))
        }
    }
}

#[derive(Debug)]
pub enum ParseSensorError {
    InvalidFormat,
}

impl FromStr for Sensor {
    type Err = ParseSensorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PATTERN: Regex =
                Regex::new(r"Sensor at x=(-*[0-9]+), y=(-*[0-9]+): closest beacon is at x=(-*[0-9]+), y=(-*[0-9]+)")
                .unwrap();
        }
        let cap = PATTERN.captures(s).ok_or(ParseSensorError::InvalidFormat)?;
        let x = cap[1].parse::<i32>().unwrap();
        let y = cap[2].parse::<i32>().unwrap();
        let beacon = Beacon {
            x: cap[3].parse::<i32>().unwrap(),
            y: cap[4].parse::<i32>().unwrap(),
        };

        let range = (x.abs_diff(beacon.x) + y.abs_diff(beacon.y)) as i32;

        Ok(Self {
            x,
            y,
            beacon,
            range,
        })
    }
}
