#[cfg(test)]
mod tests {
    use std::io::BufReader;

    #[test]
    fn test_case_1_subset() {
        let input = include_str!("./testdata/input.1.txt");
        let assignments =
            aoc_2022_d4::assignment::Assignments::from_assignments_string(input).unwrap();

        assert_eq!(assignments.count_subset(), 2);
    }

    #[test]
    fn test_case_1_overlap() {
        let input = include_str!("./testdata/input.1.txt");
        let assignments =
            aoc_2022_d4::assignment::Assignments::from_assignments_string(input).unwrap();

        assert_eq!(assignments.count_overlap(), 4);
    }

    #[test]
    fn test_case_2_subset() {
        let input = include_str!("./testdata/input.2.txt");
        let reader = BufReader::new(input.as_bytes());

        let assignments = aoc_2022_d4::assignment::Assignments::from_reader(reader).unwrap();
        assert_eq!(assignments.count_subset(), 651);
    }

    #[test]
    fn test_case_2_overlap() {
        let input = include_str!("./testdata/input.2.txt");
        let reader = BufReader::new(input.as_bytes());

        let assignments = aoc_2022_d4::assignment::Assignments::from_reader(reader).unwrap();
        assert_eq!(assignments.count_overlap(), 956);
    }
}
