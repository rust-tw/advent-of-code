fn main() {
    // read content from file input.txt
    let file_path = "input.txt"; // change this to the path of your file
    let contents = std::fs::read_to_string(file_path).expect("File Read error");

    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    // read and parse each line then put them into vector
    for line in contents.lines() {
        let (left, right) = line
            .split_once("   ")
            .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
            .unwrap();
        left_list.push(left);
        right_list.push(right);
    }

    // sort
    left_list.sort();
    right_list.sort();

    // calculate the sum of the absolute differences
    let result: u32 = left_list
        .iter()
        .zip(&right_list)
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    print!("part1: {}", result);
}
