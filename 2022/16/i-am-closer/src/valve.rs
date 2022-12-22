use std::error::Error;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Valve {
    pub flow_rate: u64,
    pub leads_to: Vec<String>,
}

#[derive(Debug)]
pub enum ParseValveError {
    InvalidFormat,
}

impl<T> From<T> for ParseValveError
where
    T: Error,
{
    fn from(_: T) -> Self {
        ParseValveError::InvalidFormat
    }
}

fn parse_valve(s: &str) -> Result<(String, u64, Vec<String>), ParseValveError> {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(
            r"Valve ([A-Z]+) has flow rate=([0-9]+); tunnel[s]* lead[s]* to valve[s]* ([A-Z, ]+)"
        )
        .unwrap();
    }
    let cap = PATTERN.captures(s).ok_or(ParseValveError::InvalidFormat)?;
    let label = cap[1].to_owned();
    let flow_rate = cap[2].parse::<u64>()?;
    let leads_to = cap[3].split(", ").map(|s| s.to_owned()).collect();

    Ok((label, flow_rate, leads_to))
}

pub fn create_routes(lines: Vec<&str>) -> Result<HashMap<String, Valve>, ParseValveError> {
    lines
        .iter()
        .map(|s| {
            let (label, flow_rate, leads_to) = parse_valve(s)?;
            Ok((
                label,
                Valve {
                    flow_rate,
                    leads_to,
                },
            ))
        })
        .collect::<Result<HashMap<_, _>, _>>()
}
