fn main() {
    // read content from file input.txt
    let file_path = "input.txt"; // change this to the path of your file
    let contents = std::fs::read_to_string(file_path).expect("File Read error");

    let mut dial: i16 = 50; // The dial starts by pointing at 50.

    let mut password_part1 = 0;
    let mut password_part2 = 0;
    for line in contents.lines() {
        let last_round_dial_is_zero = dial.eq(&0); // 前次是否停留在 0
        let mut sequence = false; // 順向R

        // 計算轉動
        if line.starts_with("R") {
            dial += line.replace("R", "").parse::<i16>().unwrap();
            sequence = true;
        } else if line.starts_with("L") {
            dial -= line.replace("L", "").parse::<i16>().unwrap();
            sequence = false;
        }

        let quotient = dial / 100; // 商，完整轉的圈數
        let remainder = dial % 100; // 餘，停留的數字
        // 如果餘數是負數，需要校正回歸
        if remainder.is_negative() {
            dial = 100 + remainder; // 校正回歸成轉盤數字
        } else {
            // 餘數為自然數就直接是轉盤的數字
            dial = remainder;
        }
        if dial.eq(&0) {
            password_part1 += 1; // 剛好停在 0，part1++
        }

        password_part2 += quotient.abs(); // 轉動的圈數就代表經過 0 的次數
        // 前次不是從 0 開始
        if !last_round_dial_is_zero
            && (
                remainder.is_negative() || // 往回轉超過 0，例如：10 L20
                (dial == 0 && !sequence) // 往回轉剛好停在 0，例如：10 L10
            )
        {
            password_part2 += 1;
        }
    }
    println!("part1: {password_part1}");
    println!("part2: {password_part2}");
}
