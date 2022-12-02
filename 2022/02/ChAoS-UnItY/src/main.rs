use std::collections::HashMap;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

fn main() {
    // Pre-computed
    let map = hashmap!(
        'A' => hashmap!(
            'X' => (3 + 1, 3 + 0),
            'Y' => (6 + 2, 1 + 3),
            'Z' => (0 + 3, 2 + 6)
        ),
        'B' => hashmap!(
            'X' => (0 + 1, 1 + 0),
            'Y' => (3 + 2, 2 + 3),
            'Z' => (6 + 3, 3 + 6)
        ),
        'C' => hashmap!(
            'X' => (6 + 1, 2 + 0),
            'Y' => (0 + 2, 3 + 3),
            'Z' => (3 + 3, 1 + 6)
        )
    );

    let data = include_str!("../Day02.txt");
    let processed_data = process_data(data);

    println!("{}", part1(&processed_data, &map));
    println!("{}", part2(&processed_data, &map));
}

fn part1(data: &[(char, char)], map: &HashMap<char, HashMap<char, (i32, i32)>>) -> i32 {
    data.iter()
        .map(|(l, r)| {
            map.get(l).unwrap().get(r).unwrap().0
        })
        .sum()
}

fn part2(data: &[(char, char)], map: &HashMap<char, HashMap<char, (i32, i32)>>) -> i32 {
    data.iter()
        .map(|(l, r)| {
            map.get(l).unwrap().get(r).unwrap().1
        })
        .sum()
}

fn process_data(data: &'static str) -> Vec<(char, char)> {
    let mut result = Vec::new();

    for line in data.replace("\r\n", "\n").split('\n') {
        let chars = line.split(' ')
            .map(|s| s.chars().next().unwrap())
            .collect::<Vec<char>>();

        result.push((chars[0], chars[1]));
    }

    result
}
