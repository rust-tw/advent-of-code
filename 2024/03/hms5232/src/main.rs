use regex::Regex;

fn main() {
    // read content from file input.txt
    let file_path = "input.txt"; // change this to the path of your file
    let contents = std::fs::read_to_string(file_path).expect("File Read error");

    /* Part 1 */

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let matches: Vec<_> = re.find_iter(&contents).map(|m| {
        m.as_str().replace("mul(", "").replace(")", "")
    }).collect();
    let part1_answer = matches.iter().map(|m| {
        // split by comma, then multiply each element
        let nums: Vec<u32> = m.split(",").map(|n| n.parse().unwrap()).collect();
        nums[0] * nums[1]
    }).sum::<u32>();

    println!("Part 1: {}", part1_answer);

    /* Part 2 */

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let matches: Vec<_> = re.find_iter(&contents).map(|m| m.as_str()).collect();
    let mut control_flag = true;
    let part2_answer: u32 = matches
        .iter()
        .map(|m| match *m {
            "do()" => {
                control_flag = true;
                return 0;
            }
            "don't()" => {
                control_flag = false;
                return 0;
            }
            _ => {
                if !control_flag {
                    return 0;
                }
                let nums: Vec<u32> = m
                    .replace("mul(", "")
                    .replace(")", "")
                    .split(",")
                    .map(|n| n.parse().unwrap())
                    .collect();
                nums[0] * nums[1]
            }
        })
        .sum();

    println!("Part 2: {}", part2_answer);
}
