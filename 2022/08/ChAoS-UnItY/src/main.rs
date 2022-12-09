use take_until::TakeUntilExt;

fn main() {
    let data = include_str!("../Day08.txt");
    let processed_data = process_data(data);

    println!("{}", part1(&processed_data));
    println!("{}", part2(&processed_data));
}

fn ranges(size: usize, start: usize) -> Vec<Vec<usize>> {
    vec![(0 .. start).rev().collect(), (start + 1 .. size).collect()]
}

fn grid_cross_walk(size: usize, start_x: usize, start_y: usize) -> Vec<Vec<usize>> {
    vec![start_x, start_y].iter()
        .flat_map(|&start| ranges(size, start))
        .collect()
}

fn part1(data: &Vec<Vec<u8>>) -> usize {
    data.iter().enumerate().flat_map(|(y, row)|
        row.iter().enumerate().map(move |(x, &height)|
            grid_cross_walk(data.len(), x, y).iter().enumerate().map(|(i, grid_walk)|
                grid_walk.iter().map(|&offset|
                    if i < 2 {
                        data[y][offset]
                    } else {
                        data[offset][x]
                    }
                ).all(|h| h < height)
            ).any(|b| b)
        ).filter(|&b| b)
    ).collect::<Vec<_>>().len()
}

fn part2(data: &Vec<Vec<u8>>) -> usize {
    data.iter().enumerate().flat_map(|(y, row)|
        row.iter().enumerate().map(move |(x, &height)|
            grid_cross_walk(data.len(), x, y).iter().enumerate().map(|(i, grid_walk)|
                grid_walk.iter().map(|&offset|
                    if i < 2 {
                        data[y][offset]
                    } else {
                        data[offset][x]
                    }
                )
                .take_until(|&h| h >= height)
                .count()
            ).fold(1, |acc, elem| acc * elem)
        )
    ).max().unwrap()
}

fn process_data(data: &'static str) -> Vec<Vec<u8>> {
    data.replace("\r\n", "\n")
        .split("\n")
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
