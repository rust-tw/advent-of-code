use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    y: usize,
    x: usize,
}

fn search_neighbor(map: &Vec<&str>, start: Point, len: usize) -> bool {
    // search the range (y, x) : (start.y - 1, start.x - 1) -> (start.y + 1, start.x + len + 1) for symbol
    #[allow(clippy::nonminimal_bool)]
    let valid_idx = |y, x| {
        y < map.len()
            && x < map[0].len()
            && ((y == start.y && !(start.x <= x && x < start.x + len)) || (y != start.y))
    };

    #[allow(clippy::needless_range_loop)]
    for y in start.y.saturating_sub(1)..=start.y.saturating_add(1) {
        for x in start.x.saturating_sub(1)..=start.x.saturating_add(len) {
            if valid_idx(y, x) && map[y].chars().nth(x).unwrap() != '.' {
                return true;
            }
        }
    }
    false
}

fn solve1(input: &str) -> i32 {
    let map: Vec<&str> = input.lines().collect();
    let mut ret = 0;
    for (y, s) in map.iter().enumerate() {
        let mut continuous_num = false;
        // [start_x, end_x)
        let mut start_x: usize = 0;
        for (x, c) in s.chars().enumerate() {
            if c.is_ascii_digit() && !continuous_num {
                continuous_num = true;
                start_x = x;
            } else if (!c.is_ascii_digit() && continuous_num)
                || (c.is_ascii_digit() && continuous_num && x == s.len() - 1)
            {
                let end_x = if !c.is_numeric() && continuous_num {
                    x
                } else {
                    x + 1
                };

                if search_neighbor(&map, Point { x: start_x, y }, end_x - start_x) {
                    ret += &s[start_x..end_x].parse::<i32>().unwrap();
                }

                start_x = 0;
                continuous_num = false;
            }
        }
    }
    ret
}

struct Pair {
    count: i32,
    v: i32,
}

fn search_neighbor_2(
    map: &Vec<&str>,
    asterisk_record: &mut HashMap<Point, Pair>,
    start: Point,
    len: usize,
) -> bool {
    // search the range (y, x) : (start.y - 1, start.x - 1) -> (start.y + 1, start.x + len + 1) for symbol

    #[allow(clippy::nonminimal_bool)]
    let valid_idx = |y, x| {
        y < map.len()
            && x < map[0].len()
            && ((y == start.y && !(start.x <= x && x < start.x + len)) || (y != start.y))
    };

    #[allow(clippy::needless_range_loop)]
    for y in start.y.saturating_sub(1)..=start.y.saturating_add(1) {
        for x in start.x.saturating_sub(1)..=start.x.saturating_add(len) {
            if valid_idx(y, x) {
                let c = map[y].chars().nth(x).unwrap();
                if c == '*' {
                    let num: i32 = map[start.y][start.x..start.x + len].parse().unwrap();
                    let ele = asterisk_record
                        .entry(Point { y, x })
                        .or_insert(Pair { count: 0, v: 1 });
                    ele.count += 1;
                    ele.v *= num;
                }
                if c != '.' {
                    return true;
                }
            }
        }
    }
    false
}

fn solve2(input: &str) -> i32 {
    let map: Vec<&str> = input.lines().collect();
    let mut asterisk_record = HashMap::new();
    let mut ret = 0;
    for (y, s) in map.iter().enumerate() {
        let mut continuous_num = false;
        // [start_x, end_x)
        let mut start_x: usize = 0;
        for (x, c) in s.chars().enumerate() {
            if c.is_ascii_digit() && !continuous_num {
                continuous_num = true;
                start_x = x;
            } else if (!c.is_ascii_digit() && continuous_num)
                || (c.is_ascii_digit() && continuous_num && x == s.len() - 1)
            {
                let end_x = if !c.is_numeric() && continuous_num {
                    x
                } else {
                    x + 1
                };
                let _ = search_neighbor_2(
                    &map,
                    &mut asterisk_record,
                    Point { x: start_x, y },
                    end_x - start_x,
                );
                start_x = 0;
                continuous_num = false;
            }
        }
    }

    for (_, v) in asterisk_record {
        if v.count > 1 {
            ret += v.v;
        }
    }
    ret
}

fn main() {
    let my_str = include_str!("input");
    println!("{}", solve1(my_str));
    println!("{}", solve2(my_str));
}
