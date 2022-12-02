use crate::{
    response::{list::ResponseList, Point},
    strategy::list::ResponseStrategyMap,
    tokenizer::{self, TheirOurTable},
};

pub struct Solution {
    table: TheirOurTable,
}

impl Solution {
    pub fn new(table: TheirOurTable) -> Self {
        Self { table }
    }

    /// 從輸入字串建構 [Solution]
    pub fn new_from_str(input: &str) -> anyhow::Result<Self> {
        let tokenized = tokenizer::tokenize(input)?;

        Ok(Self::new(tokenized))
    }

    /// 計算輸入資料的點數
    ///
    /// 這是 Question 1 的答案。
    pub fn point(&self) -> anyhow::Result<Point> {
        let response_list = ResponseList::try_from(&self.table)?;

        Ok(response_list.point())
    }

    /// 根據第二題題意計算最優解答
    pub fn find_best_solution(&self) -> anyhow::Result<ResponseList> {
        let response_strategy_map = ResponseStrategyMap::try_from(&self.table)?;

        Ok(response_strategy_map.find_best_solution())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_with_td1() {
        let input = include_str!("../testdata/td_1.txt");

        let solution = Solution::new_from_str(input).unwrap();

        assert_eq!(solution.point().unwrap(), 15);
    }

    #[test]
    fn test_point_with_td2() {
        let input = include_str!("../testdata/td_2.txt");

        let solution = Solution::new_from_str(input).unwrap();

        assert_eq!(solution.point().unwrap(), 12772);
    }

    #[test]
    fn test_sol_with_td1() {
        let input = include_str!("../testdata/td_1.txt");

        let solution = Solution::new_from_str(input).unwrap();

        assert_eq!(solution.find_best_solution().unwrap().point(), 12);
    }

    #[test]
    fn test_sol_with_td2() {
        let input = include_str!("../testdata/td_2.txt");

        let solution = Solution::new_from_str(input).unwrap();

        assert_eq!(solution.find_best_solution().unwrap().point(), 11618);
    }
}
