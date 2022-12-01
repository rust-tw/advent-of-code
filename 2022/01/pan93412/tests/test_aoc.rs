#[cfg(test)]
mod tests {
    use pan93412_aoc_2022::{AocInput, AocInputBlock};

    fn get_input_block() -> AocInputBlock {
        let i = include_str!("./testdata.txt");
        let input = AocInput::new(i);
        input.get_sum_block()
    }

    #[test]
    fn test_aoc_answer_one() {
        let blk = self::get_input_block();

        assert_eq!(blk.get_answer_one(), 24000);
    }

    #[test]
    fn test_aoc_answer_two() {
        let blk = self::get_input_block();

        assert_eq!(blk.get_answer_two(), 45000);
    }
}
