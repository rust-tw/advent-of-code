pub fn solve(content: &str)->i32 {
    let mut res = 0;
    for line in content.lines() {
        let first_digit = get_first(line);
        let last_digit = get_last(line);
        res += first_digit * 10 + last_digit;
    }
    res
}
fn get_last(line: &str)->i32 {
    line.chars().rev().find(|c| c.is_digit(10)).unwrap() as i32 - 48
}

fn get_first(line: &str)-> i32 {
    line.chars().find(|c| c.is_digit(10)).unwrap() as i32 - 48

}

#[cfg(test)]
mod tests {
    #[test]
    fn get_first() {
        assert_eq!(8, super::get_first("four82nine74"));
        assert_eq!(3, super::get_first("hlpqrdh3"));
    }
}