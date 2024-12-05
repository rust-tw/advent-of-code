#[derive(Debug)]
struct TestData {
    rules: Vec<(u32, u32)>,
    pages: Vec<Vec<u32>>,
}

impl TestData {
    fn find_precedences(&self, update: u32) -> Vec<u32> {
        self.rules
            .iter()
            .filter_map(|&(l, r)| if r == update { Some(l) } else { None })
            .collect()
    }
}

fn main() {
    let content = include_str!("../Day05.txt");
    let data = parse_content(content);
    let invalids = part1(&data);
    part2(&data, invalids);
}

fn parse_content(content: &str) -> TestData {
    let mut line_iter = content.split("\n");
    let rules = line_iter
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|rule| rule.split_once('|').unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect();
    let pages = line_iter
        .map(|line| line.split(",").map(|num| num.parse().unwrap()).collect())
        .collect();

    TestData { rules, pages }
}

fn part1(data: &TestData) -> Vec<Vec<u32>> {
    let mut invalids = vec![];
    let mut mid_page_nums = 0;

    'page: for page in &data.pages {
        for i in 0..page.len() {
            let precedences = data.find_precedences(page[i]);

            for j in i..page.len() {
                if precedences.contains(&page[j]) {
                    invalids.push(page.clone());
                    continue 'page;
                }
            }
        }

        mid_page_nums += page[page.len() / 2];
    }

    println!("Part 1: {mid_page_nums}");
    invalids
}

fn part2(data: &TestData, mut invalids: Vec<Vec<u32>>) {
    let mut mid_page_nums = 0;

    for invalid in &mut invalids {
        loop {
            let mut swapped = false;

            for i in 0..invalid.len() {
                for precedence in data.find_precedences(invalid[i]) {
                    for j in i + 1..invalid.len() {
                        if invalid[j] == precedence {
                            invalid.swap(i, j);
                            swapped = true;
                        }
                    }
                }
            }

            if !swapped {
                break;
            }
        }

        mid_page_nums += invalid[invalid.len() / 2];
    }

    println!("Part 2: {mid_page_nums}");
}
