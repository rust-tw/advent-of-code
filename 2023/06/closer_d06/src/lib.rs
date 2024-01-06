pub fn count_single_race(t: u64, d: u64) -> u64 {
    // Solve the equation: d = x(T - x)  => x^2 - tx + d = 0
    let t = t as f64;
    let d = d as f64;
    let p = (t.powi(2) - 4.0 * d).sqrt();
    assert!(!p.is_nan());

    let min = (t - p) / 2.0;
    let max = (t + p) / 2.0;

    // Integers count between these 2 solutions
    (max.ceil() - min.floor() - 1.0) as u64
}
