// 🥹
use rayon::prelude::*;

struct Almanac {
    seeds: Vec<u64>,
    resources: Vec<Vec<(u64, u64, u64)>>,
}

impl Almanac {
    fn from(input: &[String]) -> Almanac {
        let mut resources: Vec<Vec<(u64, u64, u64)>> = vec![];
        let mut i = input.iter();
        let seeds: Vec<u64> = (i.next().unwrap())
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut map: Vec<(u64, u64, u64)> = vec![];
        while let Some(line) = i.next() {
            if line.is_empty() {
                i.next();
                if !map.is_empty() {
                    map.sort_by(|(_, s, _), (_, s2, _)| s.partial_cmp(s2).unwrap());
                    resources.push(map);
                    map = vec![];
                }
                continue;
            }

            let mut l = line.split_whitespace();
            let dest: u64 = l.next().unwrap().parse().unwrap();
            let src: u64 = l.next().unwrap().parse().unwrap();
            let range: u64 = l.next().unwrap().parse().unwrap();

            map.push((dest, src, range));
        }

        if !map.is_empty() {
            map.sort_by(|(_, s, _), (_, s2, _)| s.partial_cmp(s2).unwrap());
            resources.push(map);
        }

        Almanac { seeds, resources }
    }

    fn seeds(&self) -> &Vec<u64> {
        &self.seeds
    }

    fn find_lowest_location(&self) -> u64 {
        self.find_lowest_location_with(self.seeds())
    }

    fn find_lowest_location_with(&self, seeds: &[u64]) -> u64 {
        seeds
            .par_iter()
            .map(|seed_id| {
                let mut id = *seed_id;
                for (i, _) in self.resources.iter().enumerate() {
                    let (_, src_start, _) = &self.resources[i].first().unwrap();
                    let (_, src_end, src_range) = &self.resources[i].last().unwrap();

                    if id < *src_start || id >= src_end + src_range {
                        continue;
                    }

                    for (dest, src, range) in &self.resources[i] {
                        if id >= *src && id < src + range {
                            id = id - src + dest;
                            break;
                        }
                    }
                }

                id
            })
            .min()
            .unwrap()
    }
}

pub fn solve_part1(input: &[String]) -> u64 {
    let alamanac = Almanac::from(input);

    alamanac.find_lowest_location()
}

pub fn solve_part2(input: &[String]) -> u64 {
    let alamanac = Almanac::from(input);
    let seeds = alamanac.seeds();
    let expanded_seeds: Vec<u64> = seeds
        .par_iter()
        .enumerate()
        .step_by(2)
        .flat_map(|(i, _)| seeds[i]..seeds[i] + seeds[i + 1])
        .collect();

    alamanac.find_lowest_location_with(&expanded_seeds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<String> = [
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();

        assert_eq!(solve_part1(&input), 35);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = [
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();

        assert_eq!(solve_part2(&input), 46);
    }
}
