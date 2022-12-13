use itertools::{EitherOrBoth::*, Itertools};
use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::{Chars, FromStr};

pub fn challenge_13(lines: Vec<&str>) -> (usize, usize) {
    let part1 = lines
        .split(|line| line.is_empty())
        .map(|pair| pair[0].parse::<Packet>().unwrap() < pair[1].parse::<Packet>().unwrap())
        .enumerate()
        .filter(|&(_, right_order)| right_order)
        .map(|(idx, _)| idx + 1)
        .sum();

    let mut v = lines
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Packet>().unwrap())
        .collect::<Vec<_>>();

    v.sort();

    let p1 = "[[2]]".parse::<Packet>().unwrap();
    let p2 = "[[6]]".parse::<Packet>().unwrap();

    let x1 = v.iter().take_while(|&p| p < &p1).count() + 1;
    let x2 = v.iter().take_while(|&p| p < &p2).count() + 2;

    (part1, x1 * x2)
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Value(u64),
}

#[derive(Debug, PartialEq)]
enum ParsePacketError {
    InvalidFormat,
}

impl FromStr for Packet {
    type Err = ParsePacketError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.chars().peekable();
        match iter.peek() {
            Some(&'[') => {
                iter.next();
                Self::parse_list(&mut iter)
            }
            Some(c) if c.is_numeric() => {
                iter.next();
                Self::parse_value(&mut iter)
            }
            _ => Err(Self::Err::InvalidFormat),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Value(v1), Self::Value(v2)) => v1.cmp(v2),
            (Self::List(l1), Self::List(l2)) => Itertools::zip_longest(l1.iter(), l2.iter())
                .map(|r| match r {
                    Both(p1, p2) => p1.cmp(p2),
                    Right(_) => Ordering::Less,
                    Left(_) => Ordering::Greater,
                })
                .skip_while(|ord| ord.is_eq())
                .next()
                .unwrap_or(Ordering::Equal),
            (Self::Value(v), packet @ Self::List(_)) => {
                Self::List(vec![Self::Value(*v)]).cmp(packet)
            }
            (packet @ Self::List(_), Self::Value(v)) => {
                packet.cmp(&Self::List(vec![Self::Value(*v)]))
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Packet {
    fn parse_list(chars: &mut Peekable<Chars>) -> Result<Self, ParsePacketError> {
        let mut list = Vec::new();
        while let Some(&c) = chars.peek() {
            match c {
                '[' => {
                    chars.next();
                    list.push(Self::parse_list(chars)?)
                }
                ']' => {
                    chars.next();
                    break;
                }
                ',' => {
                    chars.next();
                    continue;
                }
                x if x.is_numeric() => list.push(Self::parse_value(chars)?),
                _ => return Err(ParsePacketError::InvalidFormat),
            }
        }
        Ok(Packet::List(list))
    }

    fn parse_value(chars: &mut Peekable<Chars>) -> Result<Self, ParsePacketError> {
        let value = chars
            .take_while_ref(|c| c.is_numeric())
            .collect::<String>()
            .parse::<u64>()
            .unwrap();

        match chars.peek() {
            Some(&',') | Some(&']') | None => Ok(Packet::Value(value)),
            _ => Err(ParsePacketError::InvalidFormat),
        }
    }
}

#[test]
fn test_parse_value() {
    let s = "7],10,12";
    let mut iter = s.chars().peekable();
    assert_eq!(Packet::parse_value(&mut iter), Ok(Packet::Value(7)));
    iter.next(); // ]
    iter.next(); // ,
    assert_eq!(Packet::parse_value(&mut iter), Ok(Packet::Value(10)));
    iter.next(); // ,
    assert_eq!(Packet::parse_value(&mut iter), Ok(Packet::Value(12)));
}

#[test]
fn test_parse_list() {
    let s = "1,2,3,10,7]";
    let mut iter = s.chars().peekable();
    assert_eq!(
        Packet::parse_list(&mut iter),
        Ok(Packet::List(vec![
            Packet::Value(1),
            Packet::Value(2),
            Packet::Value(3),
            Packet::Value(10),
            Packet::Value(7)
        ]))
    );

    let s = "1,[2,3],10,7]";
    let mut iter = s.chars().peekable();
    assert_eq!(
        Packet::parse_list(&mut iter),
        Ok(Packet::List(vec![
            Packet::Value(1),
            Packet::List(vec![Packet::Value(2), Packet::Value(3)]),
            Packet::Value(10),
            Packet::Value(7)
        ]))
    );
}

#[test]
fn test_packet_cmp() {
    let data = vec![
        ("[1,1,3,1,1]", "[1,1,5,1,1]", Ordering::Less),
        ("[[1],[2,3,4]]", "[[1],4]", Ordering::Less),
        ("[9]", "[[8,7,6]]", Ordering::Greater),
        ("[[4,4],4,4]", "[[4,4],4,4,4]", Ordering::Less),
        ("[7,7,7,7]", "[7,7,7]", Ordering::Greater),
        ("[]", "[3]", Ordering::Less),
        ("[[[]]]", "[[]]", Ordering::Greater),
        (
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]",
            Ordering::Greater,
        ),
    ];
    for d in data {
        assert_eq!(
            d.0.parse::<Packet>()
                .unwrap()
                .cmp(&d.1.parse::<Packet>().unwrap()),
            d.2
        );
    }
}
