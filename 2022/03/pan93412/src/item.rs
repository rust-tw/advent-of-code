use std::fmt::Display;

use crate::InnerChar;

pub struct Item(pub InnerChar);

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}

impl Item {
    /// Get the priority of this item.
    pub fn priority(&self) -> u8 {
        // This uses some binary magic, and I think it is cool and should be documented.
        //
        // ASCII uses 7 bits to represent a character,
        // 01000001 is the ASCII code of 'A', 01011010 is the ASCII code of 'Z';
        // 01100001 is the ASCII code of 'a', 01111010 is the ASCII code of 'z'.
        //
        // Do you find any magic here? The trailing 5 bit of uppercase and
        // lowercase letters are represented the same, and the 2nd bit can
        // represent if it is uppercase or lowercase.
        //
        // Let's write the priority converter “without” any branch!

        // Extract the trailing 5 bit. 1 is 'A', 2 is 'B', ..., 26 is 'Z'.
        let letter = self.0 & 0b11111;

        // Extract the 2nd bit. 1 is lowercase, 0 is uppercase.
        // And then, we reverse this bit so it matches our name.
        let is_uppercase = (self.0 >> 5) & 1 ^ 1;

        // AoC rules:
        //
        // - Lowercase item types a through z have priorities 1 through 26.
        // - Uppercase item types A through Z have priorities 27 through 52.
        //
        // Thus, we can use the following formula to calculate the priority:
        //
        //      priority = letter + 26 * is_uppercase
        letter + 26 * is_uppercase
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(Item(b'a').priority(), 1);
        assert_eq!(Item(b'z').priority(), 26);
        assert_eq!(Item(b'A').priority(), 27);
        assert_eq!(Item(b'Z').priority(), 52);
    }
}
