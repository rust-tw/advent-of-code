type Num = usize;
type Td = (Num, Num);

fn my_parse(input: &str) -> Vec<Td> {
    let times = input
        .lines()
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<Num>().unwrap())
        .collect::<Vec<Num>>();
    let dis = input
        .lines()
        .nth(1)
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<Num>().unwrap())
        .collect::<Vec<Num>>();

    times
        .into_iter()
        .zip(dis)
        .map(|(t, d)| (t, d))
        .collect::<Vec<Td>>()
}

fn solve1(input: &str) -> Num {
    let input = my_parse(input);

    let is_perfect_square = |x: f64| -> bool { (x * x - x.round() * x.round()).abs() < 1e-5 };

    // input[i] = (time, dis)
    // (0, 0)
    // (1, (time - 1) * 1)
    // (2, (time - 2) * 2)
    // (x, (time - x) * x) -> number of x, such that time*x - x*x > dis
    // -x*x + time *x - dis > 0

    input
        .iter()
        .map(|(t, d)| {
            let delta = ((*t as f64).powf(2.0) - 4.0 * (*d as f64)).sqrt();
            let big_sol = (*t as f64 + delta) / 2.0_f64;
            let small_sol = (*t as f64 - delta) / 2.0_f64;
            let mut gap = 0;
            if is_perfect_square(big_sol) {
                gap += 1;
            }

            if is_perfect_square(small_sol) {
                gap += 1;
            }

            let big_sol = big_sol.floor() as Num;
            let small_sol = small_sol.ceil() as Num;
            big_sol - small_sol + 1 - gap
        })
        .product()
}

fn solve2(input: &str) -> Num {
    let (t, d) = my_parse(input)
        .iter()
        .fold((String::new(), String::new()), |mut acc, pair| {
            acc.0.push_str(pair.0.to_string().as_str());
            acc.1.push_str(pair.1.to_string().as_str());
            acc
        });

    let t = t.parse::<f64>().unwrap();
    let d = d.parse::<f64>().unwrap();

    let is_perfect_square = |x: f64| -> bool { (x * x - x.round() * x.round()).abs() < 1e-5 };

    // input[i] = (time, dis)
    // (0, 0)
    // (1, (time - 1) * 1)
    // (2, (time - 2) * 2)
    // (x, (time - x) * x) -> number of x, such that time*x - x*x > dis
    // -x*x + time *x - dis > 0

    let delta = (t.powf(2.0) - 4.0 * d).sqrt();
    let big_sol = (t + delta) / 2.0_f64;
    let small_sol = (t - delta) / 2.0_f64;
    let mut gap = 0;
    if is_perfect_square(big_sol) {
        gap += 1;
    }

    if is_perfect_square(small_sol) {
        gap += 1;
    }

    let big_sol = big_sol.floor() as Num;
    let small_sol = small_sol.ceil() as Num;
    big_sol - small_sol + 1 - gap
}

fn main() {
    let my_str = include_str!("input");
    println!("{}", solve1(my_str));
    println!("{}", solve2(my_str));
}
