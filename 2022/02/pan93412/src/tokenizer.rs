/// 最基本的「敵方 - 我方」Pair。
///
/// 敵方為 A, B, C；我方為 X, Y, Z。
/// 此時這個符號的意思是不確定的，
/// 得根據題目進行解析。
pub type TheirOurPair = (char, char);

/// 一堆 [TheirOurPair] Pair。
pub type TheirOurTable = Vec<TheirOurPair>;

/// 將輸入資料 tokenize 成 [TheirOurTable]。
///
/// ## 格式範例
///
/// ```plain
/// A X
/// B Y
/// C X
/// A X
/// ```
pub fn tokenize(input: &str) -> anyhow::Result<TheirOurTable> {
    let unexpected_of_line = || anyhow::anyhow!("Unexpected end of line");

    input
        .trim()
        .split('\n')
        .map(|line| {
            let mut pair = line.trim().chars();

            let left = pair.next().ok_or_else(unexpected_of_line)?;
            pair.next().ok_or_else(unexpected_of_line)?;
            let right = pair.next().ok_or_else(unexpected_of_line)?;

            Ok((left, right))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::tokenize;

    #[test]
    fn test_tokenize() {
        let input = r"A X
        B Y
        C X
        A Z";

        let expected = vec![('A', 'X'), ('B', 'Y'), ('C', 'X'), ('A', 'Z')];

        assert_eq!(tokenize(input).unwrap(), expected);
    }

    #[test]
    fn test_tokenize_failed() {
        let invalid_inputs = [
            r"A ",
            r"A",
            r"A B
              A",
            r"A B
              A ",
        ];

        for input in invalid_inputs {
            assert!(tokenize(input).is_err());
        }
    }
}
