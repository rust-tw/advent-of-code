// revised per https://github.com/rust-tw/advent-of-code/pull/86

use std::{cmp::Ordering, collections::HashMap};

type Label = u8;

enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn inject_joker(map: &mut HashMap<&u8, i32>) {
        let jokers = *map.get(&1).unwrap_or(&0);

        if jokers == 0 {
            return;
        }

        map.remove(&1);

        let (mut max_key, mut max_value) = (1, -1);

        for (&k, v) in &mut *map {
            if *v > max_value {
                max_value = *v;
                max_key = *k;
            }
        }

        match map.get_mut(&max_key) {
            Some(v) => *v += jokers,
            None => {
                map.insert(&1, jokers);
            }
        };
    }

    fn from(labels: &[Label]) -> Self {
        let mut map = HashMap::new();
        for label in labels {
            *map.entry(label).or_insert(0) += 1;
        }

        Self::inject_joker(&mut map);

        let mut values: Vec<_> = map.values().collect();
        values.sort();

        match values.len() {
            1 => Self::FiveOfAKind,
            4 => Self::OnePair,
            5 => Self::HighCard,
            2 => match values[0..2] {
                [1, 4] => Self::FourOfAKind,
                [2, 3] => Self::FullHouse,
                _ => panic!("how is this possible? 😑"),
            },
            3 => match values[0..3] {
                [1, 1, 3] => Self::ThreeOfAKind,
                [1, 2, 2] => Self::TwoPair,
                _ => panic!("how is this possible? 😑"),
            },
            _ => panic!("how is this possible? 😑"),
        }
    }

    fn rank(&self) -> u8 {
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}

struct Hand {
    hand_type: HandType,
    labels: Vec<u8>,
    bid: u32,
    use_joker: bool,
}

impl Hand {
    fn parse_label(ch: &char, use_joker: bool) -> Label {
        match ch {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => {
                if use_joker {
                    1
                } else {
                    11
                }
            }
            'T' => 10,
            '2'..='9' => *ch as u8 - b'0',
            _ => panic!("how is this possible? 😑"),
        }
    }

    fn from(input: &str, use_joker: bool) -> Self {
        let (labels, bid) = input.split_once(' ').unwrap();
        let labels: Vec<u8> = labels
            .chars()
            .map(|ch| Self::parse_label(&ch, use_joker))
            .collect();
        let hand_type = HandType::from(&labels);
        let bid = bid.parse::<u32>().unwrap();

        Self {
            labels,
            hand_type,
            bid,
            use_joker,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.hand_type.rank(), &self.labels).cmp(&(other.hand_type.rank(), &other.labels))
    }
}

impl Eq for Hand {}

pub fn solve_part1(input: &[String]) -> u32 {
    let mut hands: Vec<_> = input.iter().map(|line| Hand::from(line, false)).collect();
    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.bid)
        .sum()
}

pub fn solve_part2(input: &[String]) -> u32 {
    let mut hands: Vec<_> = input.iter().map(|line| Hand::from(line, true)).collect();
    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<String> = [
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();

        assert_eq!(solve_part1(&input), 6440);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = [
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();

        assert_eq!(solve_part2(&input), 5905);
    }
}
