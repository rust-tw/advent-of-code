use std::error::Error;
use std::str::FromStr;

pub fn challenge_04(lines: &Vec<&str>) -> u64 {
    lines
        .iter()
        .map(|&line| {
            let v = line
                .split(',')
                .map(|s| s.parse::<Section>().unwrap())
                .collect::<Vec<_>>();
            let s0 = v.get(0).unwrap();
            let s1 = v.get(1).unwrap();
            s0.contains(s1) || s1.contains(s0)
        })
        .filter(|contained| *contained)
        .count() as u64
}

#[derive(Debug, PartialEq)]
struct Section {
    begin: usize,
    end: usize,
}

impl Section {
    fn contains(&self, other: &Self) -> bool {
        self.begin <= other.begin && self.end >= other.end
    }
}

#[derive(Debug, PartialEq)]
enum ParseSectionError {
    InvalidFormat,
    InvalidValues,
}

impl<T> From<T> for ParseSectionError
where
    T: Error,
{
    fn from(_: T) -> Self {
        Self::InvalidFormat
    }
}

impl FromStr for Section {
    type Err = ParseSectionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s
            .split('-')
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;
        if let (Some(&begin), Some(&end)) = (v.get(0), v.get(1)) {
            if begin > end {
                Err(ParseSectionError::InvalidValues)
            } else {
                Ok(Section { begin, end })
            }
        } else {
            Err(ParseSectionError::InvalidFormat)
        }
    }
}

#[test]
fn test_section_construction() {
    assert_eq!("2-4".parse(), Ok(Section { begin: 2, end: 4 }));
    assert_eq!("11-24".parse(), Ok(Section { begin: 11, end: 24 }));
    assert_eq!(
        "7-4".parse::<Section>(),
        Err(ParseSectionError::InvalidValues)
    );
    assert_eq!(
        "2-b".parse::<Section>(),
        Err(ParseSectionError::InvalidFormat)
    );
}

#[test]
fn test_if_contains() {
    let test = |s1: &str, s2: &str| {
        s1.parse::<Section>()
            .unwrap()
            .contains(&(s2.parse::<Section>().unwrap()))
    };
    assert!(!test("2-4", "6-8"));
    assert!(test("2-8", "3-7"));
    assert!(test("4-6", "6-6"));
}
