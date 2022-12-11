use lazy_static::lazy_static;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Arithmetic {
    Add(i32),
    Multiply(i32),
    Square,
}

impl Arithmetic {
    pub fn calculate(&self, a: i32) -> i32 {
        match *self {
            Arithmetic::Add(b) => a + b,
            Arithmetic::Multiply(b) => a * b,
            Arithmetic::Square => a * a,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Monkey {
    pub items: Vec<i32>,
    pub operation: Arithmetic,
    pub test: i32,
    pub true_to: i32,
    pub false_to: i32,
}

#[derive(Debug, PartialEq, Eq)]
enum InputToken {
    NewMoneky,
    Items(Vec<i32>),
    Operation(Arithmetic),
    Test(i32),
    TrueTo(i32),
    FalseTo(i32),
}

struct Grammar {
    pattern: regex::Regex,
    capture: fn(regex::Captures) -> InputToken,
}

lazy_static! {
    static ref ITEMS: Grammar = Grammar {
        pattern: regex::Regex::from_str(r#"Starting items:(.*)$"#).unwrap(),
        capture: |capture| {
            InputToken::Items(
                capture
                    .get(1)
                    .unwrap()
                    .as_str()
                    .split(",")
                    .map(|s| s.trim())
                    .map(|digits| digits.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            )
        },
    };
    static ref OPERATION: Grammar = Grammar {
        pattern: regex::Regex::from_str(r#"Operation: new = old ([*+]) (\d+|old)"#).unwrap(),
        capture: |capture| {
            let operand = capture.get(2).unwrap().as_str();
            let operand = match operand {
                "old" => return InputToken::Operation(Arithmetic::Square),
                _ => operand.parse().unwrap(),
            };
            InputToken::Operation(match capture.get(1).unwrap().as_str() {
                "+" => Arithmetic::Add(operand),
                "*" => Arithmetic::Multiply(operand),
                _ => panic!("unsupproted arithemtic"),
            })
        }
    };
    static ref TEST: Grammar = Grammar {
        pattern: regex::Regex::from_str(r#"Test:.*?(\d+)"#).unwrap(),
        capture: |capture| InputToken::Test(capture.get(1).unwrap().as_str().parse().unwrap())
    };
    static ref TRUE_TO: Grammar = Grammar {
        pattern: regex::Regex::from_str(r#"If true:.*?(\d+)"#).unwrap(),
        capture: |capture| InputToken::TrueTo(capture.get(1).unwrap().as_str().parse().unwrap()),
    };
    static ref FALSE_TO: Grammar = Grammar {
        pattern: regex::Regex::from_str(r#"If false:.*?(\d+)"#).unwrap(),
        capture: |capture| InputToken::FalseTo(capture.get(1).unwrap().as_str().parse().unwrap()),
    };
    static ref GRAMMARS: [&'static Grammar; 5] = [&ITEMS, &OPERATION, &TEST, &TRUE_TO, &FALSE_TO];
}

fn tokenize(line: impl AsRef<str>) -> InputToken {
    for g in GRAMMARS.iter() {
        if let Some(captures) = g.pattern.captures(line.as_ref()) {
            return (g.capture)(captures);
        }
    }
    InputToken::NewMoneky
}

pub fn parse_monkeys<'a, S: AsRef<str>>(input: impl Iterator<Item = S>) -> Vec<Monkey> {
    let mut items = None;
    let mut operation = None;
    let mut test = None;
    let mut true_to = None;
    let mut false_to = None;
    let mut monkeys = vec![];
    for token in input
        .filter(|line| !line.as_ref().is_empty())
        .map(tokenize)
        .skip(1)
    {
        match token {
            InputToken::Items(parsed) => items = Some(parsed),
            InputToken::Test(parsed) => test = Some(parsed),
            InputToken::TrueTo(parsed) => true_to = Some(parsed),
            InputToken::FalseTo(parsed) => false_to = Some(parsed),
            InputToken::Operation(parsed) => operation = Some(parsed),
            InputToken::NewMoneky => monkeys.push(Monkey {
                items: items.take().unwrap(),
                operation: operation.take().unwrap(),
                test: test.take().unwrap(),
                true_to: true_to.take().unwrap(),
                false_to: false_to.take().unwrap(),
            }),
        }
    }
    monkeys.push(Monkey {
        items: items.take().unwrap(),
        operation: operation.take().unwrap(),
        test: test.take().unwrap(),
        true_to: true_to.take().unwrap(),
        false_to: false_to.take().unwrap(),
    });
    monkeys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_1() {
        let tokens = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3"#
            .lines()
            .map(tokenize)
            .collect::<Vec<InputToken>>();
        assert_eq!(
            vec![
                InputToken::NewMoneky,
                InputToken::Items(vec![79, 98]),
                InputToken::Operation(Arithmetic::Multiply(19)),
                InputToken::Test(23),
                InputToken::TrueTo(2),
                InputToken::FalseTo(3),
            ],
            tokens
        );
    }

    #[test]
    fn test_parse_monkeys() {
        assert_eq!(
            vec![
                Monkey {
                    items: vec![79, 98],
                    operation: Arithmetic::Multiply(19),
                    test: 23,
                    true_to: 2,
                    false_to: 3,
                },
                Monkey {
                    items: vec![54, 65, 75, 74],
                    operation: Arithmetic::Add(6),
                    test: 19,
                    true_to: 2,
                    false_to: 0,
                },
                Monkey {
                    items: vec![79, 60, 97],
                    operation: Arithmetic::Square,
                    test: 13,
                    true_to: 1,
                    false_to: 3,
                },
                Monkey {
                    items: vec![74],
                    operation: Arithmetic::Add(3),
                    test: 17,
                    true_to: 0,
                    false_to: 1,
                }
            ],
            parse_monkeys(include_str!("testdata/example.txt").lines())
        )
    }
}
