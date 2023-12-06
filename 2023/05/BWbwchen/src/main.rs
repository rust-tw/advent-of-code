// reference: https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/05.rs
// reference: https://github.com/jonathanpaulson/AdventOfCode/blob/master/2023/5.py#L63

type Num = usize;
struct Layer {
    dst: Num,
    src: Num,
    range: Num,
}
impl Layer {
    fn contains(&self, x: Num) -> bool {
        (self.src..self.src + self.range).contains(&x)
    }
    fn converts(&self, x: Num) -> Num {
        self.dst + x - self.src
    }
}

fn my_parse(input: &str) -> (Vec<Num>, Vec<Vec<Layer>>) {
    let (seed, layer) = input.split_once("\n\n").unwrap();
    let seed = seed
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<Num>().unwrap())
        .collect::<Vec<Num>>();
    let layers = layer
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .skip(1)
                .filter(|s| !s.is_empty())
                .map(|m| {
                    let tuple = m
                        .split(' ')
                        .map(|x| x.parse::<Num>().unwrap())
                        .collect::<Vec<Num>>();
                    let mut tuple = tuple.iter();
                    Layer {
                        dst: *tuple.next().unwrap(),
                        src: *tuple.next().unwrap(),
                        range: *tuple.next().unwrap(),
                    }
                })
                .collect::<Vec<Layer>>()
        })
        .collect::<Vec<_>>();
    (seed, layers)
}

fn solve1(input: &str) -> Num {
    let (seed, layers) = my_parse(input);
    let result = layers.iter().fold(seed, |seed, layer| {
        seed.into_iter()
            .map(|s| {
                layer
                    .iter()
                    .find(|l| l.contains(s))
                    .map(|l| l.converts(s))
                    .unwrap_or(s)
            })
            .collect()
    });
    *result.iter().min().unwrap()
}

// [start, end)
struct Range {
    start: Num,
    end: Num,
}

impl From<(Num, Num)> for Range {
    fn from(value: (Num, Num)) -> Self {
        Range {
            start: value.0,
            end: value.1,
        }
    }
}

impl Range {
    fn valid(&self) -> bool {
        self.start < self.end
    }
}

fn solve2(input: &str) -> Num {
    let (seed, layers) = my_parse(input);

    let mut group_seed = Vec::with_capacity(seed.len());
    for i in (0..seed.len()).step_by(2) {
        group_seed.push(Range {
            start: seed[i],
            end: seed[i] + seed[i + 1],
        });
    }
    let seed = group_seed;

    let result = layers.iter().fold(seed, |seed, layer| {
        seed.into_iter()
            .flat_map(|s| {
                // start ------------------------------------ end
                //              src -------- src+range
                // [ unchanged1][     changed        ][unchanged2]
                let mut ret_map: Vec<Range> = Vec::new();
                let mut un_map: Vec<Range> = vec![s];

                for l in layer {
                    let mut new_unmap: Vec<Range> = Vec::new();
                    for m in un_map {
                        let start = m.start;
                        let end = m.end;
                        // If [start, end) all in range unchanged1
                        let unchanged1: Range = (start, end.min(l.src)).into();
                        // If [start, end) all in range
                        let changed: Range = (start.max(l.src), end.min(l.src + l.range)).into();
                        // If [start, end) all in range unchanged2
                        let unchanged2: Range = ((l.src + l.range).max(start), end).into();

                        if unchanged1.valid() {
                            new_unmap.push(unchanged1);
                        }
                        if changed.valid() {
                            ret_map
                                .push((l.converts(changed.start), l.converts(changed.end)).into());
                        }
                        if unchanged2.valid() {
                            new_unmap.push(unchanged2);
                        }
                    }
                    un_map = new_unmap;
                }
                ret_map.extend(un_map); // from the un_map part, remember to record it.
                ret_map
            })
            .collect()
    });
    result.iter().map(|s| s.start).min().unwrap()
}

fn main() {
    let my_str = include_str!("input");
    println!("{}", solve1(my_str));
    println!("{}", solve2(my_str));
}
