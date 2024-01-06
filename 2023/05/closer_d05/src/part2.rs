use closer_d05::*;

pub fn solve(lines: &Vec<&str>) -> u64 {
    let mut blocks = lines.split(|line| line.is_empty());

    let seeds: Vec<_> = parse_seeds(blocks.next().unwrap()[0])
        .chunks(2)
        .map(|x| SeedRange::new(x[0], x[1]))
        .collect();

    let mappings = blocks
        .map(|block| Mapping::parse(block))
        .collect::<Vec<_>>();

    //---- Brutal-force solution. For verification only. Not suitable for large data.
    // let v = seeds.iter()
    //     .map(|seed_range| {
    //         (seed_range.begin..(seed_range.begin + seed_range.length))
    //             .map(|seed|{
    //                 mappings
    //                 .iter()
    //                 .fold(seed, |acc, mapping| mapping.convert(acc))
    //             })
    //             .collect::<Vec<_>>()
    //         })
    //     .flatten()
    //     .collect::<Vec<_>>();
    // println!("{v:?}");

    seeds
        .into_iter()
        .map(|seed_range| {
            mappings.iter().fold(vec![seed_range], |acc, mapping| {
                acc.into_iter()
                    .map(|sr| mapping.convert_range(sr))
                    .flatten()
                    .collect::<Vec<_>>()
            })
        })
        .flatten()
        .min()
        .unwrap()
        .begin
}
