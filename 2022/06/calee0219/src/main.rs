use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path = "test.input";
    let contents = fs::read_to_string(file_path)
        .expect("Read file error");
    println!("{:?}", part(contents.clone(), 4));
    println!("{:?}", part(contents.clone(), 14));
}

fn part(contents: String, window: usize) -> Vec<i32> {
    let mut ret = Vec::new();
    for ln in contents.lines() {
        let mut map = HashMap::new();
        for (idx, ch) in ln.chars().enumerate() {
            *map.entry(ch).or_insert(0) += 1;
            if idx >= window {
                let prev = ln.chars().nth(idx-window).unwrap();
                *map.get_mut(&prev).unwrap() -= 1;
                if map[&prev] == 0 {
                    map.remove(&prev);
                }
            }
            if map.len() == window {
                ret.push((idx+1) as i32);
                break;
            }
        }
    }
    ret
}
