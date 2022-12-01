#[cfg(test)]
mod tests {
    use pan93412_aoc_2022::AocInput;

    #[test]
    fn test_aoc() {
        let i = include_str!("./testdata.txt");
        let input = AocInput::new(i);
        let blk = input.get_sum_block();

        assert_eq!(blk.get_answer_one(), 24000);
        assert_eq!(blk.get_answer_two(), 45000);
    }
}
