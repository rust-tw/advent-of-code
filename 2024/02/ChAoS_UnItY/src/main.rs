fn main() {
    let content = include_str!("../Day02.txt");
    part1(content);
    part2(content);
}

fn part1(content: &str) {
     let safe_cnt = content.split("\n")
        .map(|line| line.split(" ").map(|num| num.parse::<u32>().unwrap()).collect::<Vec<_>>())
        .filter(|nums| {
            let inc = nums[0] < nums[1];
            let mut prev = nums[0];

            for &num in nums.into_iter().skip(1) {
                if (inc && prev > num) || (!inc && prev < num) || prev.abs_diff(num) > 3 || prev.abs_diff(num) == 0 {
                    return false;
                }

                prev = num;
            }

            return true;
        })
        .count();

    println!("Part 1: {safe_cnt}");
}

fn part2(content: &str) {
    let safe_cnt = content.split("\n")
        .map(|line| line.split(" ").map(|num| num.parse::<u32>().unwrap()).collect::<Vec<_>>())
        .filter(|nums| {
            'outer: for i in 0..nums.len() {
                let mut nums = nums.clone();
                nums.remove(i);

                let inc = nums[0] < nums[1];
                let mut prev = nums[0];

                for num in nums.into_iter().skip(1) {
                    if (inc && prev > num) || (!inc && prev < num) || prev.abs_diff(num) > 3 || prev.abs_diff(num) == 0 {
                        continue 'outer;
                    }

                    prev = num;
                }

                return true;
            }

            return false;
        })
        .count();

    println!("Part 2: {safe_cnt}");
}
