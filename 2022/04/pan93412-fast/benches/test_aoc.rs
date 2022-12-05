#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use aoc_2022_d4::assignment::Assignments;
    use paste::paste;
    use test::{black_box, Bencher};

    macro_rules! construct_bencher {
        ($name:ident, $action:ident, $input:expr) => {
            paste! {
                #[bench]
                fn [<bench_ $name _ $action>](b: &mut Bencher) {
                    [<bench_base_ $action>](b, include_str!($input))
                }
            }
        };

        ($name:ident, $input:expr) => {
            construct_bencher!($name, read_string, $input);
            construct_bencher!($name, bufread, $input);
            construct_bencher!($name, subset, $input);
            construct_bencher!($name, overlap, $input);
        };
    }

    construct_bencher!(small_input, "../tests/testdata/input.1.txt");
    construct_bencher!(big_input, "../tests/testdata/input.2.txt");

    fn bench_base_read_string(b: &mut Bencher, input: &str) {
        b.iter(|| {
            Assignments::from_assignments_string(black_box(input)).unwrap();
        })
    }

    #[inline]
    fn bench_base_bufread(b: &mut Bencher, input: &str) {
        use std::io::BufReader;

        b.iter(|| {
            let reader = BufReader::new(input.as_bytes());
            Assignments::from_reader(black_box(reader)).unwrap();
        })
    }

    #[inline]
    fn bench_base_subset(b: &mut Bencher, input: &str) {
        let assignment = Assignments::from_assignments_string(input).unwrap();

        b.iter(|| {
            black_box(&assignment).count_subset();
        })
    }

    #[inline]
    fn bench_base_overlap(b: &mut Bencher, input: &str) {
        let assignment = Assignments::from_assignments_string(input).unwrap();

        b.iter(|| {
            black_box(&assignment).count_overlap();
        })
    }
}
