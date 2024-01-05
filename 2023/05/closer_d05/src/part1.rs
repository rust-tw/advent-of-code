use closer_d05::*;

pub fn solve(lines: &Vec<&str>) -> u64 {
    let mut blocks = lines.split(|line| line.is_empty());

    let seeds = parse_seeds(blocks.next().unwrap()[0]);

    let mappings = blocks
        .map(|block| Mapping::parse(block))
        .collect::<Vec<_>>();

    seeds
        .into_iter()
        .map(|seed| {
            mappings
                .iter()
                .fold(seed, |acc, mapping| mapping.convert(acc))
        })
        .min()
        .unwrap_or(0)
}
