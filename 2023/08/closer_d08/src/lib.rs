use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub fn parse_data(lines: &Vec<&str>) -> (Vec<usize>, HashMap<[char; 3], [[char; 3]; 2]>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([A-Z1-9]+) = \(([A-Z1-9]+), ([A-Z1-9]+)\)").unwrap();
    }
    let steps = lines[0]
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect();
    let map = HashMap::from_iter(lines[2..].iter().map(|s| {
        let captures = RE.captures(s).unwrap();
        let key = (&captures[1])
            .chars()
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap();
        let left = (&captures[2])
            .chars()
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap();
        let right = (&captures[3])
            .chars()
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap();
        (key, [left, right])
    }));
    (steps, map)
}
