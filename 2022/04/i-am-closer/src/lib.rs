use std::error::Error;
use std::str::FromStr;

pub fn challenge_04(lines: Vec<&str>) -> (u64, u64) {
    let v = lines
        .into_iter()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<Section>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part1 = v
        .iter()
        .map(|sections| sections[0].contains(&sections[1]) || sections[1].contains(&sections[0]))
        .filter(|x| *x)
        .count() as u64;

    let part2 = v
        .iter()
        .map(|sections| sections[0].overlaps(&sections[1]))
        .filter(|x| *x)
        .count() as u64;

    (part1, part2)
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

    fn overlaps(&self, other: &Self) -> bool {
        if self.begin < other.begin {
            self.end >= other.begin
        } else if self.begin > other.begin {
            other.end >= self.begin
        } else {
            true
        }
    }
}

#[derive(Debug, PartialEq)]
enum ParseSectionError {
    InvalidFormat,
    InvalidValues,
}

// 利用這個 trait，將所有有作 Error trait 的 Error type 都能轉成 ParseSectionError。
// 這樣下面在使用時，就可以用 `?` operator 將任意的 Error type 轉成 ParseSectionError。
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

#[test]
fn test_if_overlaps() {
    let test = |s1: &str, s2: &str| {
        s1.parse::<Section>()
            .unwrap()
            .overlaps(&(s2.parse::<Section>().unwrap()))
    };

    assert!(!test("2-4", "6-8"));
    assert!(!test("4-5", "2-3"));
    // TODO: 應該要有更全面的 test cases。
    assert!(test("1-10", "4-7"), "left contains right");
    assert!(test("2-9", "1-10"), "right contains left");
    assert!(test("1-7", "3-10"), "left tail overlaps right head");
    assert!(test("3-10", "1-7"), "left head overlaps right tail");
}
