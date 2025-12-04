fn main() {
    // read content from file input.txt
    let file_path = "input.txt"; // change this to the path of your file
    let contents = std::fs::read_to_string(file_path).expect("File Read error");

    // 先切出各個 range
    let ranges = contents.lines().next().unwrap();
    let mut part1 = 0;
    ranges.split(",").for_each(|range| {
        let (start, end) = range.split_once("-").unwrap();
        // 迴圈所有數字
        for i in start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap() {
            // 取得數字長度
            let len = i.to_string().len();
            // 判斷位數是否為偶數
            if len % 2 == 0 {
                // 除數，從數字中間一刀切用
                let divisor = 10usize.pow((len / 2) as u32);
                // 商和餘相等 => 重複自己的數字
                if i / divisor == i % divisor {
                    part1 += i;
                }
            }
        }
    });
    println!("Part 1: {}", part1);
}
