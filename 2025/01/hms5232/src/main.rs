fn main() {
    // read content from file input.txt
    let file_path = "input.txt"; // change this to the path of your file
    let contents = std::fs::read_to_string(file_path).expect("File Read error");

    let mut dial: i16 = 50; // The dial starts by pointing at 50.

    let mut password_part1 = 0;
    for line in contents.lines() {
        // 計算轉動
        if line.starts_with("R") {
            dial += line.replace("R", "").parse::<i16>().unwrap();
        } else if line.starts_with("L") {
            dial -= line.replace("L", "").parse::<i16>().unwrap();
        }

        let remainder = dial % 100; // 餘，停留的數字
        // 如果餘數是負數，需要校正回歸
        if remainder.is_negative() {
            dial = 100 + remainder; // 校正回歸成轉盤數字
        } else {
            // 餘數為正就直接是轉盤的數字
            dial = remainder;
        }
        if dial.eq(&0) {
            password_part1 += 1; // 剛好停在 0，part1++
        }
    }
    println!("part1: {password_part1}");
}
