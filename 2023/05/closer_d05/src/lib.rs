use std::collections::VecDeque;

pub fn parse_seeds(s: &str) -> Vec<u64> {
    s.split(':')
        .skip(1)
        .next()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

pub struct Mapping {
    rules: Vec<MappingItem>,
}

impl Mapping {
    pub fn parse(block: &[&str]) -> Self {
        Mapping {
            rules: block
                .iter()
                .skip(1)
                .map(|s| MappingItem::new_with_str(s))
                .collect(),
        }
    }

    pub fn convert(&self, orig: u64) -> u64 {
        self.rules
            .iter()
            .map(|mapping_item| mapping_item.try_convert(orig))
            .find(|dest| dest.is_some())
            .flatten()
            .unwrap_or(orig)
    }

    pub fn convert_range(&self, orig: SeedRange) -> Vec<SeedRange> {
        let mut deque = VecDeque::new();
        let mut result = Vec::new();
        deque.push_back(orig);

        'seed_queue: while !deque.is_empty() {
            let seedrange = deque.pop_front().unwrap();
            for rule in &self.rules {
                let (in_range, out_range) = rule.try_convert_range(seedrange);
                if !in_range.is_empty() {
                    result.extend(in_range.iter());
                    deque.extend(out_range.iter());
                    continue 'seed_queue;
                }
            }
            result.push(seedrange);
        }

        result
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord)]
pub struct SeedRange {
    pub begin: u64,
    pub length: u64,
}

impl SeedRange {
    pub fn new(begin: u64, length: u64) -> Self {
        Self { begin, length }
    }

    fn apply_delta(self, delta: i64) -> Self {
        Self {
            begin: (self.begin as i64 + delta) as u64,
            length: self.length,
        }
    }
}

impl PartialOrd for SeedRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.begin.cmp(&other.begin))
    }
}

enum RangeStatus {
    Less,
    Hit,
    Greater,
}

struct MappingItem {
    begin: u64,
    end: u64,
    delta: i64,
}

impl MappingItem {
    fn new_with_str(s: &str) -> Self {
        let numbers = s
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|part| part.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        MappingItem {
            begin: numbers[1],
            end: numbers[1] + numbers[2],
            delta: (numbers[0] as i64) - (numbers[1] as i64),
        }
    }

    fn test_range(&self, pos: u64) -> RangeStatus {
        if pos < self.begin {
            RangeStatus::Less
        } else if pos < self.end {
            RangeStatus::Hit
        } else {
            RangeStatus::Greater
        }
    }

    fn try_convert(&self, src: u64) -> Option<u64> {
        if let RangeStatus::Hit = self.test_range(src) {
            Some((src as i64 + self.delta) as u64)
        } else {
            None
        }
    }

    fn try_convert_range(&self, src: SeedRange) -> (Vec<SeedRange>, Vec<SeedRange>) {
        match (
            self.test_range(src.begin),
            self.test_range(src.begin + src.length - 1),
        ) {
            (RangeStatus::Less, RangeStatus::Less) | 
            (RangeStatus::Greater, RangeStatus::Greater) => (vec![], vec![src]),
            (RangeStatus::Hit, RangeStatus::Hit) => {
                (vec![src.apply_delta(self.delta)], vec![])
            }
            (RangeStatus::Hit, RangeStatus::Greater) => {
                let in_range = SeedRange::new(src.begin, self.end - src.begin);
                let out_range = SeedRange::new(src.begin + in_range.length, src.length - in_range.length);
                (vec![in_range.apply_delta(self.delta)], vec![out_range])
            }
            (RangeStatus::Less, RangeStatus::Hit) => {
                let out_range = SeedRange::new(src.begin, self.begin - src.begin);
                let in_range = SeedRange::new(self.begin, src.length - out_range.length);
                (vec![in_range.apply_delta(self.delta)], vec![out_range])
            }
            (RangeStatus::Less, RangeStatus::Greater) => {
                // println!("Seed: {}..{}; Map: {}..{}", src.begin, src.begin + src.length, self.begin, self.end);
                let out_range1 = SeedRange::new(src.begin, self.begin - src.begin);
                let in_range = SeedRange::new(self.begin, self.end - self.begin);
                let out_range2 = SeedRange::new(self.end, src.length - out_range1.length - in_range.length);
                (vec![in_range.apply_delta(self.delta)], vec![out_range1, out_range2])
            }
            _ => {
                panic!("Tail > Head... Impossible!!")
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    fn create_single_rule_mapping() -> Mapping {
        Mapping {
            rules: vec![MappingItem::new_with_str("5 10 10")],
        }
    }

    #[test]
    fn out_of_range() {
        assert_eq!(
            create_single_rule_mapping().convert_range(SeedRange::new(1, 5)),
            vec![SeedRange::new(1, 5)]
        );
    }

    #[test]
    fn out_of_range_but_connected() {
        assert_eq!(
            create_single_rule_mapping().convert_range(SeedRange::new(5, 5)),
            vec![SeedRange::new(5, 5)]
        );
    }

    #[test]
    fn tail_intersection() {
        assert_eq!(
            create_single_rule_mapping().convert_range(SeedRange::new(5, 6)),
            vec![SeedRange::new(5, 1), SeedRange::new(5, 5)]
        );
    
    }

    #[test]
    fn within_range() {
        assert_eq!(
            create_single_rule_mapping().convert_range(SeedRange::new(10, 5)),
            vec![SeedRange::new(5, 5)]
        );
    }

    #[test]
    fn head_intersection() {
        assert_eq!(
            create_single_rule_mapping().convert_range(SeedRange::new(17, 10)),
            vec![SeedRange::new(12, 3), SeedRange::new(20, 7)]
        );
    }

    #[test]
    fn seed_range_covers_rule() {
        assert_eq!(
            create_single_rule_mapping().convert_range(SeedRange::new(5, 20)),
            vec![SeedRange::new(5, 10), SeedRange::new(5, 5), SeedRange::new(20, 5)]
        )
    }
}
