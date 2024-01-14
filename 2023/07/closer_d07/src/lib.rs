use core::panic;
use std::cmp::Ordering;

pub fn solve_with_rule(lines: &Vec<&str>, new_rule: bool) -> u64 {
    let mut plays: Vec<_> = lines
        .iter()
        .map(|s| {
            let tokens = s.split(' ').filter(|s| !s.is_empty()).collect::<Vec<_>>();
            let hand = Hand::from_str(tokens[0], new_rule);
            let bet = tokens[1].parse::<u64>().unwrap();
            (hand, bet)
        })
        .collect();

    plays.sort();

    plays
        .into_iter()
        .enumerate()
        .map(|(idx, (_, bet))| (idx as u64 + 1) * bet)
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
    handtype: HandType,
}

impl Hand {
    pub fn from_str(s: &str, new_rule: bool) -> Self {
        let mut cs = s.chars();
        let cards = std::array::from_fn(|_| Card::from_char(cs.next().unwrap(), new_rule));

        Hand {
            cards,
            handtype: Hand::judge_type(&cards, new_rule),
        }
    }

    fn judge_type(cards: &[Card], new_rule: bool) -> HandType {
        let mut counts = [0_usize; 15];
        cards
            .iter()
            .for_each(|Card(point)| counts[(*point) as usize] += 1);

        let handtype = counts
            .into_iter()
            .skip(2)
            .fold(HandType::HighCard, |cur_type, count| match count {
                5 => HandType::FiveOfAKind,
                4 => HandType::FourOfAKind,
                3 => match cur_type {
                    HandType::HighCard => HandType::ThreeOfAKind,
                    HandType::OnePair => HandType::FullHouse,
                    _ => cur_type,
                },
                2 => match cur_type {
                    HandType::HighCard => HandType::OnePair,
                    HandType::OnePair => HandType::TwoPair,
                    HandType::ThreeOfAKind => HandType::FullHouse,
                    _ => cur_type,
                },
                _ => cur_type,
            });

        if new_rule {
            match counts[1] {
                5 | 4 => HandType::FiveOfAKind,
                3 => match handtype {
                    HandType::OnePair => HandType::FiveOfAKind,
                    HandType::HighCard => HandType::FourOfAKind,
                    _ => handtype,
                },
                2 => match handtype {
                    HandType::ThreeOfAKind => HandType::FiveOfAKind,
                    HandType::OnePair => HandType::FourOfAKind,
                    HandType::HighCard => HandType::ThreeOfAKind,
                    _ => handtype,
                },
                1 => match handtype {
                    HandType::FourOfAKind => HandType::FiveOfAKind,
                    HandType::ThreeOfAKind => HandType::FourOfAKind,
                    HandType::TwoPair => HandType::FullHouse,
                    HandType::OnePair => HandType::ThreeOfAKind,
                    HandType::HighCard => HandType::OnePair,
                    _ => handtype,
                },
                _ => handtype,
            }
        } else {
            handtype
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.handtype.cmp(&other.handtype) {
            Ordering::Equal => {
                let cmp_cards =
                    self.cards
                        .iter()
                        .zip(other.cards.iter())
                        .find_map(|(a, b)| match a.cmp(b) {
                            Ordering::Equal => None,
                            ord => Some(ord),
                        });

                match cmp_cards {
                    Some(ord) => ord,
                    None => Ordering::Equal,
                }
            }
            ord => ord,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
struct Card(u64);

impl Card {
    fn from_char(c: char, new_rule: bool) -> Self {
        match c {
            '2'..='9' => Self((c as u64) - ('0' as u64)),
            'T' => Self(10),
            'J' => {
                if new_rule {
                    Self(1)
                } else {
                    Self(11)
                }
            }
            'Q' => Self(12),
            'K' => Self(13),
            'A' => Self(14),
            _ => panic!("Invalid value!!"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[test]
fn compare_hands() {
    assert!(Hand::from_str("22A22", false) > Hand::from_str("AAKKT", false));
    assert!(Hand::from_str("33332", false) > Hand::from_str("2AAAA", false));
    assert!(Hand::from_str("77888", false) > Hand::from_str("77788", false));
    assert!(Hand::from_str("77888", false) == Hand::from_str("77888", false));
}
