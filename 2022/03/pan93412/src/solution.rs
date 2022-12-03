use crate::rucksack::Rucksacks;

pub struct Solution<'a>(Rucksacks<'a>);

impl<'a> Solution<'a> {
    pub fn new(input: &'a str) -> Self {
        Self(Rucksacks::from(input))
    }

    /// Get the answer of Q1.
    pub fn solve_question_1(&self) -> u32 {
        self.0
            .to_compartment_pairs()
            .filter_map(|pair| pair.find_common_item())
            .map(|item| item.priority() as u32)
            .sum()
    }

    /// Get the answer of Q2.
    pub fn solve_question_2(&self) -> u32 {
        self.0
            .to_groups::<3>()
            .iter()
            .filter_map(|pair| pair.find_common_item())
            .map(|item| item.priority() as u32)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_solve_q1_with_td1() {
        let s = Solution::new(include_str!("../testdata/td1.txt"));
        assert_eq!(s.solve_question_1(), 157);
    }

    #[test]
    fn test_solve_q1_with_td2() {
        let s = Solution::new(include_str!("../testdata/td2.txt"));
        assert_eq!(s.solve_question_1(), 7793);
    }

    #[test]
    fn test_solve_q2_with_td1() {
        let s = Solution::new(include_str!("../testdata/td1.txt"));
        assert_eq!(s.solve_question_2(), 70);
    }

    #[test]
    fn test_solve_q2_with_td2() {
        let s = Solution::new(include_str!("../testdata/td2.txt"));
        assert_eq!(s.solve_question_2(), 2499);
    }
}
