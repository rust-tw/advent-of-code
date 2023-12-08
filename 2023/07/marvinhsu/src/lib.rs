use std::collections::HashMap;

use anyhow::{anyhow, Ok, Result};
use itertools::Itertools;
#[allow(unused_imports)]
use tap::Tap;

pub fn part1(input: &str) -> Result<usize> {
    let result = input
        .lines()
        .map(|line| {
            let (head, tail) = line
                .split_once(' ')
                .ok_or(anyhow!("Parse fail: {}", line))?;
            let bet_number = tail.parse::<usize>()?;
            let labels: [Label; 5] = head
                .chars()
                .map(|c| Label::from_char_part1(&c))
                .process_results(|iter| iter.collect_vec())?
                .try_into()
                .map_err(|_| anyhow!("Parse Labels Fail:{}", line))?;

            let card_type = CardType::from_labels_part1(labels);

            Ok((bet_number, card_type))
        })
        .process_results(|iter| {
            iter.sorted_by_key(|card| card.1)
                .enumerate()
                .map(|(round, card)| (round + 1) * card.0)
                .sum()
        })?;

    Ok(result)
}

pub fn part2(input: &str) -> Result<usize> {
    let result = input
        .lines()
        .map(|line| {
            let (head, tail) = line
                .split_once(' ')
                .ok_or(anyhow!("Parse fail: {}", line))?;
            let bet_number = tail.parse::<usize>()?;
            let labels: [Label; 5] = head
                .chars()
                .map(|c| Label::from_char_part2(&c))
                .process_results(|iter| iter.collect_vec())?
                .try_into()
                .map_err(|_| anyhow!("Parse Labels Fail:{}", line))?;

            let card_type = CardType::from_labels_part2(labels);

            Ok((bet_number, card_type))
        })
        .process_results(|iter| {
            iter.sorted_by_key(|card| card.1)
                .enumerate()
                .map(|(round, card)| (round + 1) * card.0)
                .sum()
        })?;

    Ok(result)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Label {
    Jocker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Label {
    fn from_char_part1(c: &char) -> Result<Self> {
        match c {
            '2' => Ok(Label::Two),
            '3' => Ok(Label::Three),
            '4' => Ok(Label::Four),
            '5' => Ok(Label::Five),
            '6' => Ok(Label::Six),
            '7' => Ok(Label::Seven),
            '8' => Ok(Label::Eight),
            '9' => Ok(Label::Nine),
            'T' => Ok(Label::Ten),
            'J' => Ok(Label::Jack),
            'Q' => Ok(Label::Queen),
            'K' => Ok(Label::King),
            'A' => Ok(Label::Ace),
            _ => Err(anyhow::anyhow!("Invalid label: {}", c)),
        }
    }

    fn from_char_part2(c: &char) -> Result<Self> {
        match c {
            '2' => Ok(Label::Two),
            '3' => Ok(Label::Three),
            '4' => Ok(Label::Four),
            '5' => Ok(Label::Five),
            '6' => Ok(Label::Six),
            '7' => Ok(Label::Seven),
            '8' => Ok(Label::Eight),
            '9' => Ok(Label::Nine),
            'T' => Ok(Label::Ten),
            'J' => Ok(Label::Jocker),
            'Q' => Ok(Label::Queen),
            'K' => Ok(Label::King),
            'A' => Ok(Label::Ace),
            _ => Err(anyhow::anyhow!("Invalid label: {}", c)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CardType {
    HighCard([Label; 5]),
    OnePair([Label; 5]),
    TwoPair([Label; 5]),
    ThreeKind([Label; 5]),
    FullHouse([Label; 5]),
    FourKind([Label; 5]),
    FiveKind([Label; 5]),
}

impl CardType {
    fn from_labels_part1(labels: [Label; 5]) -> Self {
        let map = labels.iter().fold(HashMap::new(), |mut map, label| {
            *map.entry(label).or_insert(0) += 1;
            map
        });

        let set = map.iter().map(|(_, v)| v).sorted().collect_vec();

        match set.as_slice() {
            [5] => CardType::FiveKind(labels),
            [1, 4] => CardType::FourKind(labels),
            [2, 3] => CardType::FullHouse(labels),
            [1, 1, 3] => CardType::ThreeKind(labels),
            [1, 2, 2] => CardType::TwoPair(labels),
            [1, 1, 1, 2] => CardType::OnePair(labels),
            _ => CardType::HighCard(labels),
        }
    }

    /// needed debug
    fn from_labels_part2(labels: [Label; 5]) -> Self {
        let (map, jockers) =
            labels
                .iter()
                .fold((HashMap::new(), vec![]), |(mut map, mut jockers), label| {
                    if *label == Label::Jocker {
                        jockers.push(label);
                    } else {
                        *map.entry(label).or_insert(0) += 1;
                    }

                    (map, jockers)
                });

        let set = map.iter().map(|(_, v)| v).sorted().collect_vec();
        let jocker_count = jockers.len();

        match (set.as_slice(), jocker_count) {
            (l, 0) => match l {
                [5] => CardType::FiveKind(labels),
                [1, 4] => CardType::FourKind(labels),
                [2, 3] => CardType::FullHouse(labels),
                [1, 1, 3] => CardType::ThreeKind(labels),
                [1, 2, 2] => CardType::TwoPair(labels),
                [1, 1, 1, 2] => CardType::OnePair(labels),
                _ => CardType::HighCard(labels),
            },
            (l, 1) => match l {
                [4] => CardType::FiveKind(labels),
                [1, 3] => CardType::FourKind(labels),
                [2, 2] => CardType::FullHouse(labels),
                [1, 1, 2] => CardType::ThreeKind(labels),
                _ => CardType::OnePair(labels),
            },
            (l, 2) => match l {
                [3] => CardType::FiveKind(labels),
                [1, 2] => CardType::FourKind(labels),
                _ => CardType::ThreeKind(labels),
            },
            (l, 3) => match l {
                [2] => CardType::FiveKind(labels),
                _ => CardType::FourKind(labels),
            },
            _ => CardType::FiveKind(labels),
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{part1, part2, CardType, Label};

    use super::{CardType::*, Label::*};

    #[test]
    fn test_part_1() {
        let input = include_str!("../example");
        assert_eq!(part1(input).unwrap(), 6440);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../example");
        assert_eq!(part2(input).unwrap(), 5905);
    }

    #[test]
    fn test_compare_label() {
        assert!(Ace > King);
        assert!(King > Queen);
        assert!(Queen > Jack);
        assert!(Jack > Ten);
        assert!(Ten > Nine);
        assert!(Nine > Eight);
        assert!(Eight > Seven);
        assert!(Seven > Six);
        assert!(Six > Five);
        assert!(Five > Four);
        assert!(Four > Three);
        assert!(Three > Two);
    }

    #[test]
    fn test_compare_card_type() {
        // 32T3K 765
        // T55J5 684
        // KK677 28
        // KTJJT 220
        // QQQJA 483

        let input = [
            OnePair([Three, Two, Ten, Three, King]),
            ThreeKind([Ten, Five, Five, Jack, Five]),
            TwoPair([King, King, Six, Seven, Seven]),
            TwoPair([King, Ten, Jack, Jack, Ten]),
            ThreeKind([Queen, Queen, Queen, Jack, Ace]),
        ]
        .iter()
        .enumerate()
        .sorted_by_key(|card| card.1)
        .map(|card| card.0)
        .collect_vec();

        assert_eq!(input, [0, 3, 2, 1, 4]);
    }

    #[test]
    fn test_parse_label() {
        assert_eq!(Label::from_char_part1(&'2').unwrap(), Two);
        assert_eq!(Label::from_char_part1(&'3').unwrap(), Three);
        assert_eq!(Label::from_char_part1(&'J').unwrap(), Jack);

        assert_eq!(Label::from_char_part2(&'2').unwrap(), Two);
        assert_eq!(Label::from_char_part2(&'3').unwrap(), Three);
        assert_eq!(Label::from_char_part2(&'J').unwrap(), Jocker);
    }

    #[test]
    fn test_parse_card_type_part2() {
        // 32T3K 765
        // T55J5 684
        // KK677 28
        // KTJJT 220
        // QQQJA 483

        assert_eq!(
            CardType::from_labels_part2([Three, Two, Ten, Three, King]),
            OnePair([Three, Two, Ten, Three, King])
        );

        assert_eq!(
            CardType::from_labels_part2([Ten, Five, Five, Jocker, Five]),
            FourKind([Ten, Five, Five, Jocker, Five])
        );

        assert_eq!(
            CardType::from_labels_part2([King, King, Six, Seven, Seven]),
            TwoPair([King, King, Six, Seven, Seven])
        );

        assert_eq!(
            CardType::from_labels_part2([King, Ten, Jocker, Jocker, Ten]),
            FourKind([King, Ten, Jocker, Jocker, Ten])
        );

        assert_eq!(
            CardType::from_labels_part2([Queen, Queen, Queen, Jocker, Ace]),
            FourKind([Queen, Queen, Queen, Jocker, Ace])
        );
    }

    // #[test]
    // fn test_parse_card_type() {
    //     assert_eq!(
    //         "32T3K".parse::<CardType>().unwrap(),
    //         OnePair([Three, Two, Ten, Three, King])
    //     );
    //     assert_eq!(
    //         "76584".parse::<CardType>().unwrap(),
    //         HighCard([Seven, Six, Five, Eight, Four])
    //     );
    //     assert_eq!(
    //         "KK677".parse::<CardType>().unwrap(),
    //         TwoPair([King, King, Six, Seven, Seven])
    //     );
    //     assert_eq!(
    //         "KTJJT".parse::<CardType>().unwrap(),
    //         TwoPair([King, Ten, Jack, Jack, Ten])
    //     );
    //     assert_eq!(
    //         "QQQJA".parse::<CardType>().unwrap(),
    //         ThreeKind([Queen, Queen, Queen, Jack, Ace])
    //     );
    // }
}
