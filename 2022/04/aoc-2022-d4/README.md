# Pan's solution of AoC 2022 Day 4

The performance-oriented and well-engineered solution.

## Run

```bash
cargo run --release your_filename.txt
```

## Bench Result

```plain
test tests::bench_big_input_bufread       ... bench:     676,825 ns/iter (+/- 18,606)
test tests::bench_big_input_overlap       ... bench:      17,349 ns/iter (+/- 1,398)
test tests::bench_big_input_read_string   ... bench:     632,610 ns/iter (+/- 8,138)
test tests::bench_big_input_subset        ... bench:      65,178 ns/iter (+/- 3,553)
test tests::bench_small_input_bufread     ... bench:       1,589 ns/iter (+/- 68)
test tests::bench_small_input_overlap     ... bench:          38 ns/iter (+/- 1)
test tests::bench_small_input_read_string ... bench:       1,375 ns/iter (+/- 37)
test tests::bench_small_input_subset      ... bench:          42 ns/iter (+/- 0)
```

## File Hierarchy

- `benches`
  - `test_aoc.rs`: Bench our functions
- `examples`
  - `bench_test.rs`: The example of this library (also for profiling).
- `src`
  - `main.rs`: CLI interface
  - `lib.rs`: AoC 2022 D4 library
  - `assignment.rs`: See <https://adventofcode.com/2022/day/4>
  - `section.rs`: See <https://adventofcode.com/2022/day/4>
- `tests`
  - `testdata`: A bunch of raw testdata.
  - `test_actual_case`: Integration Test.

## SCC

```plain
───────────────────────────────────────────────────────────────────────────────
Language                 Files     Lines   Blanks  Comments     Code Complexity
───────────────────────────────────────────────────────────────────────────────
JSON                       221       221        0         0      221          0
D                          192      1864      466         0     1398          0
Rust                        11       526       92        17      417         22
Plain Text                   4      1016        0         0     1016          0
TOML                         3        45       11         0       34          0
LLVM IR                      2        16        4         2       10          0
Markdown                     1        66       11         0       55          0
───────────────────────────────────────────────────────────────────────────────
Total                      434      3754      584        19     3151         22
───────────────────────────────────────────────────────────────────────────────
Estimated Cost to Develop (organic) $90,150
Estimated Schedule Effort (organic) 5.51 months
Estimated People Required (organic) 1.45
───────────────────────────────────────────────────────────────────────────────
Processed 468649 bytes, 0.469 megabytes (SI)
───────────────────────────────────────────────────────────────────────────────
```

## License

`AGPL-3.0-only`
