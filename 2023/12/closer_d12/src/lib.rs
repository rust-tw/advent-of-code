pub fn can_fit(cs: &[char], length: usize) -> bool {
    cs.len() > length && cs.iter().take(length).map(|&c| c != '.').all(|b| b) && cs[length] != '#'
}
