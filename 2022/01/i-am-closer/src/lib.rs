pub fn challenge_01(lines: &Vec<&str>) -> Option<(u32, u32)> {
    let calories_per_elf: Vec<_> = lines
        .split(|&line| line == "")
        .map(|backpack| {
            backpack
                .iter()
                .map(|&food| food.parse::<u32>().unwrap_or(0))
                .sum::<u32>()
        })
        .collect();

    // Part 1
    let max_calories = *calories_per_elf.iter().max().unwrap();

    // Part 2
    let top_3_calories = calories_per_elf
        .iter()
        .fold((0, 0, 0), |top3, &calorie| match calorie {
            c if calorie > top3.0 => (c, top3.0, top3.1),
            c if calorie > top3.1 => (top3.0, c, top3.1),
            c if calorie > top3.2 => (top3.0, top3.1, c),
            _ => top3,
        });

    Some((
        max_calories,
        top_3_calories.0 + top_3_calories.1 + top_3_calories.2,
    ))
}

#[test]
fn short_data() {
    let lines = vec!["1000", "2000", "3000", "", "4000", "", "5000", "6000"];
    assert_eq!(challenge_01(&lines), Some((11000, 21000)));
}
