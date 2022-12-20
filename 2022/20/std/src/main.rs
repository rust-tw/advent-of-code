fn main() {
    let input = include_str!("../input.txt");
    let file: Vec<isize> = input.lines().map(|s| s.parse().unwrap()).collect();

    let part1 = mixing(&file, 1, 1);
    println!("{part1}");

    let part2 = mixing(&file, 10, 811589153);
    println!("{part2}");
}

fn mixing(file: &[isize], times: usize, key: isize) -> isize {
    let mut file = file.iter().enumerate().map(|(i, v)| (i, v * key)).collect::<Vec<_>>();
    let len = file.len();

    for _ in 0..times {
        for index in 0..len {
            let current_index = file.iter().position(|&(i, _)| i == index).unwrap();
            let num = file.remove(current_index);
            let new_index = (num.1 + current_index as isize).rem_euclid(len as isize - 1);
            file.insert(new_index as usize, num);
        }
    }

    let index = file.iter().position(|&(_, x)| x == 0).unwrap();
    [1000, 2000, 3000].iter().fold(0, |acc, i| acc + file[(i + index) % len].1)
}
