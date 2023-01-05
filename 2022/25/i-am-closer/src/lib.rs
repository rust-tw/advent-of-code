mod snafu;

pub use snafu::Snafu;

pub fn challenge_25(lines: Vec<&str>) -> (String, usize) {
    let fuel_req: i64 = lines.iter().map(|line| {
        i64::from(line.parse::<Snafu>().unwrap())
    })
    .sum();

    let part1 = Snafu::from(fuel_req).to_string();

    (part1, 0)
}
