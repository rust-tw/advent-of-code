use std::collections::HashSet;

pub fn match_count(card_text: &str) -> usize {
    let mut number_sets_iter = card_text
    .split(':')
    .skip(1)
    .next()
    .unwrap()
    .split('|')
    .map(|list| {
        list.split(' ')
            .filter(|s| !s.is_empty())
            .map(|part| part.trim().parse::<i32>().unwrap())
            .collect::<HashSet<_>>()
    });

    let winning_numbers = number_sets_iter.next().unwrap();
    let my_numbers = number_sets_iter.next().unwrap();

    my_numbers.intersection(&winning_numbers).count()
}
