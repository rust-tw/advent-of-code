use std::str::FromStr;

pub fn challenge_05(lines: Vec<&str>) -> (String, String) {
    let stacks_data = lines
        .iter()
        .cloned()
        .take_while(|&line| !line.is_empty())
        .collect::<Vec<_>>();
    let mut stacks_part1 = Stacks::parse(stacks_data);
    let mut stacks_part2 = stacks_part1.clone();

    if let Ok(actions) = lines
        .iter()
        .skip_while(|line| !line.is_empty())
        .skip(1) // skip the empty line
        .map(|line| line.parse::<Action>())
        .collect::<Result<Vec<_>, ParseActionError>>()
    {
        actions
            .iter()
            .for_each(|action| {
                stacks_part1.apply_cm9000(&action);
                stacks_part2.apply_cm9001(&action);
            });
        (stacks_part1.compose_top(), stacks_part2.compose_top())
    } else {
        // TODO: handle parse errors...
        (String::new(), String::new())
    }

}

#[derive(Debug, PartialEq, Clone)]
struct Stacks(Vec<Vec<char>>);

impl Stacks {
    #[allow(dead_code)]
    fn new() -> Self {
        Self::with_count(0)
    }

    fn with_count(count: usize) -> Self {
        Stacks(vec![vec![]; count])
    }

    fn parse(lines: Vec<&str>) -> Stacks {
        let stack_count = lines
            .iter()
            .rev()
            .next()
            .unwrap()
            .split(' ')
            .filter(|&s| !s.is_empty())
            .count();

        let mut stacks = Stacks::with_count(stack_count);
        lines.into_iter().rev().skip(1).for_each(|line| {
            line.as_bytes()
                .chunks(4)
                .enumerate()
                .for_each(|(index, token)| {
                    if token[1] != (' ' as u8) {
                        stacks.push_to(index, token[1] as char);
                    }
                })
        });
        stacks
    }

    fn push_to(&mut self, index: usize, value: char) {
        self.0[index].push(value)
    }

    fn apply_cm9000(&mut self, action: &Action) {
        let v: Vec<char> = (0..(action.count))
            .map(|_| self.0[action.from].pop().unwrap())
            .collect();
        v.iter().for_each(|c| self.0[action.to].push(*c));
    }

    fn apply_cm9001(&mut self, action: &Action) {
        let mut v: Vec<_> = (0..(action.count))
            .map(|_| self.0[action.from].pop().unwrap())
            .collect();
        v.reverse();
        v.iter().for_each(|c| self.0[action.to].push(*c));
    }

    fn compose_top(&self) -> String {
        self.0.iter().map(|v| v.last().unwrap_or(&' ')).collect()
    }
}

#[test]
fn test_parse_stacks() {
    let lines = vec!["    [D]    ", "[N] [C]    ", "[Z] [M] [P]", " 1   2   3 "];
    assert_eq!(
        Stacks::parse(lines),
        Stacks(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']])
    );
}

#[derive(Debug, PartialEq)]
struct Action {
    from: usize,
    to: usize,
    count: usize,
}

#[derive(Debug)]
enum ParseActionError {
    InvalidFormat,
}

impl<T> From<T> for ParseActionError
where
    T: std::error::Error,
{
    fn from(_: T) -> Self {
        Self::InvalidFormat
    }
}

impl FromStr for Action {
    type Err = ParseActionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split(' ').collect::<Vec<_>>();
        if tokens.len() != 6 {
            return Err(ParseActionError::InvalidFormat);
        }
        let count = tokens[1].parse::<usize>()?;
        let from = tokens[3].parse::<usize>()? - 1;
        let to = tokens[5].parse::<usize>()? - 1;
        Ok(Action { from, to, count })
    }
}

#[test]
fn test_parse_action() {
    // TODO: add more test cases, include parsing errors.
    assert_eq!(
        "move 1 from 5 to 8".parse::<Action>().unwrap(),
        Action {
            from: 4,
            to: 7,
            count: 1
        }
    );
}
