const LIMIT: (u32, u32, u32) = (12, 13, 14); // (R, G, B)

fn solve1(input: &str) -> u32 {
    input.lines().fold(0, |acc, s| {
        let form = s.split(':').collect::<Vec<&str>>();
        let id = form[0].split(' ').last().unwrap().parse::<u32>().unwrap();
        let content = form[1]
            .trim()
            .split(';')
            .map(|s| {
                let mut valid = true;
                s.trim().split(',').for_each(|s| {
                    let vv = s.trim().split(' ').collect::<Vec<&str>>();
                    let count = vv[0].parse::<u32>().unwrap();
                    match vv[1] {
                        "green" => {
                            valid &= count <= LIMIT.1;
                        }
                        "red" => {
                            valid &= count <= LIMIT.0;
                        }
                        "blue" => {
                            valid &= count <= LIMIT.2;
                        }
                        _ => panic!(),
                    }
                });
                valid
            })
            .collect::<Vec<bool>>();
        let valid = content
            .iter()
            .fold(0, |acc, v| if *v { acc + 1 } else { acc });

        if valid as usize == content.len() {
            acc + id
        } else {
            acc
        }
    })
}

fn solve2(input: &str) -> u32 {
    input.lines().fold(0, |acc, s| {
        let form = s.split(':').collect::<Vec<&str>>();
        let content = form[1]
            .trim()
            .split(';')
            .map(|s| {
                let mut record: (u32, u32, u32) = (u32::MIN, u32::MIN, u32::MIN);
                s.trim().split(',').for_each(|s| {
                    let vv = s.trim().split(' ').collect::<Vec<&str>>();
                    let count = vv[0].parse::<u32>().unwrap();
                    match vv[1] {
                        "green" => {
                            record.1 = record.1.max(count);
                        }
                        "red" => {
                            record.0 = record.0.max(count);
                        }
                        "blue" => {
                            record.2 = record.2.max(count);
                        }
                        _ => panic!(),
                    }
                });
                record
            })
            .fold((u32::MIN, u32::MIN, u32::MIN), |acc, x| {
                (
                    std::cmp::max(acc.0, x.0),
                    std::cmp::max(acc.1, x.1),
                    std::cmp::max(acc.2, x.2),
                )
            });

        acc + content.0 * content.1 * content.2
    })
}

fn main() {
    let my_str = include_str!("day02-input");
    println!("{}", solve1(my_str));
    println!("{}", solve2(my_str));
}
