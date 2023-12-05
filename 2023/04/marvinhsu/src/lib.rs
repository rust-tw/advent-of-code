use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub fn day4_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<ScratchCard>().map(|card| card.get_points()))
        .sum::<Result<_, _>>()
        .expect("Failed to parse scratch card")
}

pub fn day4_part2(input: &str) -> usize {
    let cards: Vec<ScratchCard> = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()
        .expect("Failed to parse scratch card");

    let cards_len = cards.len();

    cards
        .iter()
        .fold(HashMap::new(), |mut map, card| {
            let count = card.count_matching_numbers();
            let instances = map.entry(card.id).or_insert(0);
            *instances += 1;

            if count > 0 {
                let instances = *instances;

                for id in (card.id + 1)..=(card.id + count).min(cards_len) {
                    *map.entry(id).or_insert(0) += instances;
                }
            }
            map
        })
        .values()
        .sum()
}

#[derive(Debug, PartialEq, Clone)]
struct ScratchCard {
    id: usize,
    wining_numbers: HashSet<usize>,
    scratch_numbers: HashSet<usize>,
}

impl ScratchCard {
    fn count_matching_numbers(&self) -> usize {
        self.wining_numbers
            .intersection(&self.scratch_numbers)
            .count()
    }

    fn get_points(&self) -> usize {
        self.count_matching_numbers()
            .checked_add_signed(-1)
            .map(|pow| 2_usize.pow(pow as u32))
            .unwrap_or(0)
    }
}

impl FromStr for ScratchCard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(':')
            .and_then(|(head, tail)| {
                let id: usize = head
                    .split_once(' ')
                    .and_then(|(_, id)| id.trim().parse().ok())?;

                let (wining_numbers, scratch_numbers) =
                    tail.split_once('|').map(|(head, tail)| {
                        (
                            head.split(' ')
                                .filter_map(|n| n.trim().parse().ok())
                                .collect(),
                            tail.split(' ')
                                .filter_map(|n| n.trim().parse().ok())
                                .collect(),
                        )
                    })?;

                Some(Self {
                    id,
                    wining_numbers,
                    scratch_numbers,
                })
            })
            .ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("../example");

        assert_eq!(day4_part1(input), 13);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../example");

        assert_eq!(day4_part2(input), 30);
    }

    #[test]
    fn test_parse_scratch_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let card = input.parse::<ScratchCard>().unwrap();

        assert_eq!(
            card,
            ScratchCard {
                id: 1,
                wining_numbers: vec![41, 48, 83, 86, 17].into_iter().collect(),
                scratch_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53].into_iter().collect(),
            }
        );
    }
}
