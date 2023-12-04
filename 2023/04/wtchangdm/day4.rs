use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct Card {
    id: u32,
    winning: HashSet<u32>,
    owned: HashSet<u32>,
}

fn parse_numbers(s: &str) -> HashSet<u32> {
    s.split_whitespace()
        .map(|c| c.parse::<u32>().unwrap())
        .collect()
}

impl Card {
    fn from(line: &str) -> Self {
        let split: Vec<&str> = line.split(": ").collect();
        let id: u32 = split[0].replace("Card ", "").trim().parse().unwrap();
        let numbers: Vec<&str> = split[1].split(" | ").collect();
        let winning: HashSet<u32> = parse_numbers(numbers[0]);
        let owned: HashSet<u32> = parse_numbers(numbers[1]);

        Self { id, winning, owned }
    }

    fn points(&self) -> u32 {
        match self
            .winning
            .intersection(&self.owned)
            .count()
            .checked_sub(1)
        {
            None => 0, // negative, like -1
            Some(v) => 2_u32.pow(v as u32),
        }
    }

    fn won_copies(&self) -> usize {
        self.winning.intersection(&self.owned).count()
    }
}

pub fn solve_part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| Card::from(line))
        .map(|card| card.points())
        .sum()
}

pub fn solve_part2(input: &[String]) -> u32 {
    let mut count: HashMap<u32, u32> = HashMap::new();
    let mut copies: HashMap<u32, usize> = HashMap::new();
    let mut queue: VecDeque<u32> = VecDeque::new();

    input.iter().map(|line| Card::from(line)).for_each(|card| {
        count.entry(card.id).or_insert(1);
        copies.entry(card.id).or_insert(card.won_copies());

        (1..=card.won_copies())
            .map(|i| i as u32 + card.id)
            .for_each(|id| queue.push_back(id));
    });

    while !queue.is_empty() {
        let card_id = queue.pop_front().unwrap();
        *count.get_mut(&card_id).unwrap() += 1;

        (1..=*copies.get(&card_id).unwrap())
            .map(|i| i as u32 + card_id)
            .for_each(|id| queue.push_back(id));
    }

    count.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<String> = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();

        assert_eq!(solve_part1(&input), 13);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();

        assert_eq!(solve_part2(&input), 30);
    }
}
