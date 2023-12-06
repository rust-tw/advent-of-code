use std::collections::HashMap;

pub fn solve(contents: &str)->u32 {
    let mut res = 0;
    for line in contents.lines() {
        res += calibration_value(line);
    }
    res
}

fn calibration_value(line: &str)-> u32 {
    let iter = 0..line.len();
        let first_digit = find_num(line, 0..line.len());
        let last_digit = find_num(line, iter.rev());
        first_digit * 10 + last_digit
}

fn find_num<'a, I>(line: &str, iter: I)-> u32
where
    I: Iterator<Item = usize>,
{
    for i in iter {
        match get_num(&line[i..]) {
            Some(num) => return num,
            None => (),
        }
    }
   0
}

fn get_num(line: &str) -> Option<u32> {
    let mut m: HashMap<&str, u32> = HashMap::new();
    m.insert("one", 1);
    m.insert("two", 2);
    m.insert("three", 3);
    m.insert("four", 4);
    m.insert("five", 5);
    m.insert("six", 6);
    m.insert("seven",7);
    m.insert("eight", 8);
    m.insert("nine", 9);
    if line.starts_with(char::is_numeric) {
        return line.chars().next().unwrap().to_digit(10)
    }
    for (k, v) in m {
        if line.starts_with(k) {
            return Some(v)
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calibration_value() {
        assert_eq!(29, calibration_value("two1nine"));
        assert_eq!(83, calibration_value("eightwothree"));
        assert_eq!(13, calibration_value("abcone2threexyz"));
        assert_eq!(24, calibration_value("xtwone3four"));
        assert_eq!(42, calibration_value("4nineeightseven2"));
        assert_eq!(14, calibration_value("zoneight234"));
        assert_eq!(76, calibration_value("7pqrstsixteen"));
    }
}