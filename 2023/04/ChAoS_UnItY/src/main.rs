fn main() {
    let content = include_str!("../Day04.txt");
    let games = resolve_input(content);
    part1(&games);
    part2(&games);
}

fn resolve_numbers(segments: &str) -> Vec<u32> {
    segments
        .split(" ")
        .filter(|&candidate| !candidate.is_empty())
        .map(|number| number.parse::<u32>().unwrap())
        .collect()
}

fn resolve_input(content: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    content
        .split("\n")
        .map(|line| {
            line.split_once(": ")
                .unwrap()
                .1
                .split_once(" | ")
                .map(|(winnings, holds)| (resolve_numbers(winnings), resolve_numbers(holds)))
                .unwrap()
        })
        .collect()
}

fn part1(games: &[(Vec<u32>, Vec<u32>)]) {
    let mut result = 0;

    for (winnings, holds) in games {
        let matches = holds.iter().filter(|&hold| winnings.contains(hold)).count();

        result += if matches == 0 {
            0
        } else {
            2u32.pow(matches as u32 - 1)
        };
    }

    println!("{result}");
}

fn part2(games: &[(Vec<u32>, Vec<u32>)]) {
    let mut cards = vec![1; games.len()];

    for (index, (winnings, holds)) in games.iter().enumerate() {
        let base = cards[index];
        let matches = holds.iter().filter(|&hold| winnings.contains(hold)).count();
        cards[index + 1..games.len().min(index + 1 + matches)]
            .iter_mut()
            .for_each(|target_base| *target_base += base);
    }

    let result = cards.iter().sum::<u32>();

    println!("{result}");
}
