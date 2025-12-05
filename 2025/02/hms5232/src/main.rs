fn main() {
    // read content from file input.txt
    let file_path = "input.txt"; // change this to the path of your file
    let contents = std::fs::read_to_string(file_path).expect("File Read error");

    // 先切出各個 range
    let ranges = contents.lines().next().unwrap();
    let mut part1 = 0;
    let mut part2 = 0;
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

            /* part 2 */
            // 延續 part1 切割思考，採用除法方式，但如果想簡潔其實也能 [0..1] 這種方式切？
            {
                // 位數小於 2 不可能符合條件，直接跳過
                if len < 2 {
                    continue;
                }
                // 先算出切半後最靠近的位數
                let mut current_digit = len / 2;
                // 每個位數一一檢查
                while current_digit > 0 {
                    // 切割的位數必須整除數字位數才有可能符合條件
                    if len % current_digit == 0 {
                        let base = i % 10usize.pow(current_digit as u32); // 被用來比較的數字
                        // 每個切割位數經過降級後和 base 比較，不同的話就可以直接下一回合
                        let mut remain = i; // 從左邊開始切，切完後剩下的
                        let mut check = true;
                        let mut zero_count = len - current_digit; // 比較的 0 個數
                        // 檢查每個切割出來的數字是否都和 base 相同
                        for _j in 1..(len / current_digit) {
                            let divisor = 10usize.pow(zero_count as u32);
                            // 不同的話就可以直接下一位數
                            if remain / divisor != base {
                                check = false;
                                break;
                            }
                            // 相同的話則繼續切和比較
                            remain = remain % divisor;
                            zero_count -= current_digit;
                        }
                        if check {
                            part2 += i;
                            break;
                        }
                    }
                    current_digit -= 1;
                }
            }
        }
    });
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
