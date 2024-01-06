pub fn make_schematic(lines: &Vec<&str>) -> Vec<Vec<char>> {
    let height = lines.len();
    if height == 0 {
        return vec![];
    }
    let width = lines[0].len();

    let mut schematic = vec![vec!['.'; width + 2]; height + 2];
    for h in 0..height {
        for (w, c) in lines[h].chars().enumerate() {
            schematic[h + 1][w + 1] = c;
        }
    }

    schematic
}
