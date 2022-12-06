#![no_std]

pub fn find_marker(input: &str, length: usize) -> usize {
    input
        .as_bytes()
        .windows(length)
        .enumerate()
        .find(|(_, window)| {
            window
                .iter()
                .fold(0usize, |acc, x| (acc | (1 << (x - b'a'))))
                .count_ones()
                == length as u32
        })
        .map(|(i, _)| i + length)
        .unwrap()
}
