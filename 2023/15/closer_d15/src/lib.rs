use std::str::FromStr;

pub fn hashing(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .fold(0, |acc, b| (acc + (*b as usize)) * 17 % 256)
}

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Remove(String),
    Update(String, u64),
}

impl Op {
    pub fn label(&self) -> &str {
        match self {
            Op::Remove(s) => s.as_str(),
            Op::Update(s, _) => s.as_str(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseOpError;

impl FromStr for Op {
    type Err = ParseOpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split('=').collect::<Vec<_>>();
        if v.len() == 2
            && v[1]
                .chars()
                .all(|c| c.is_ascii_digit() && v[0].chars().all(|c| c.is_ascii_alphabetic()))
        {
            Ok(Op::Update(v[0].to_owned(), v[1].parse::<u64>().unwrap()))
        } else if s.chars().last().unwrap() == '-' {
            Ok(Op::Remove(
                s.chars().take_while(|c| *c != '-').collect::<String>(),
            ))
        } else {
            Err(ParseOpError)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hash_str() {
        assert_eq!(hashing("rn=1"), 30);
        assert_eq!(hashing("cm-"), 253);
        assert_eq!(hashing("qp=3"), 97);
        assert_eq!(hashing("cm=2"), 47);
        assert_eq!(hashing("qp-"), 14);
        assert_eq!(hashing("pc=4"), 180);
        assert_eq!(hashing("ot=9"), 9);
    }
}
