use std::{ops::RangeInclusive, collections::HashSet};

fn main() {
    let data = include_str!("../Day04.txt");
    let processed_data = process_data(data);

    println!("{}", part1(&processed_data));
    println!("{}", part2(&processed_data));
}

fn part1(data: &Vec<Vec<RangeInclusive<i32>>>) -> i32 {
    data.iter()
        .map(|ranges| {
            let [l_range, r_range] = into_ranges(ranges);
            let intersected = l_range.intersection(&r_range).collect::<Vec<_>>();
        
            if intersected.len() == l_range.len() || intersected.len() == r_range.len() {
                1
            } else {
                0
            }
        })
        .sum()
}

fn part2(data: &Vec<Vec<RangeInclusive<i32>>>) -> i32 {
    data.iter()
        .map(|ranges| {
            let [l_range, r_range] = into_ranges(ranges);
            l_range.intersection(&r_range)
                .collect::<Vec<_>>()
                .len()
        })
        .filter(|len| *len > 0)
        .collect::<Vec<_>>()
        .len() as i32
}

fn into_ranges(ranges: &Vec<RangeInclusive<i32>>) -> [HashSet<i32>; 2] {
    ranges.iter()
        .map(|range| HashSet::from_iter(range.clone().into_iter()))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn process_data(data: &'static str) -> Vec<Vec<RangeInclusive<i32>>> {
    data.replace("\r\n", "\n")
        .split('\n')
        .map(|line| {
            line.split(',')
                .map(|range| {
                    let mut range_points = range.split('-')
                        .map(|i| i.parse::<i32>().unwrap());

                    range_points.next().unwrap()..=range_points.next().unwrap()
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
