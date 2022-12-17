// #![no_std]
use scapegoat::SgMap;

// Please edit these two constant to meet your need.
const INPUT_LEN: usize = 10092;
const RECORD_LEN: usize = 2000;

const WIDTH: usize = 7;
const HEIGHT: usize = 10000;
const MAX: usize = 1000000000000;
struct Chameber([u8; WIDTH * HEIGHT]);
type Rock = [u8; 16];

impl Chameber {
    fn fit(&self, rock: &Rock, x: usize, y: usize) -> bool {
        rock.iter().enumerate().all(|(i, r)| {
            self.0[(y + i / 4) * WIDTH + (x + i % 4)] == 0
            || *r == 0
        })
    }

    fn place(&mut self, rock: &Rock, x: usize, y: usize) {
        rock.iter().enumerate().for_each(|(i, r)| {
            if *r == 1 {
                self.0[(y + i / 4) * WIDTH + (x + i % 4)] = 1;
            }
        })
    }

    fn snapshot(&self, i: usize) -> usize {
        self.0[i-64..i].iter().fold(0, |v,i|  v | (1 << i) )
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut gas = [true; INPUT_LEN];
    input.trim().chars().enumerate().for_each(|(i, c)| gas[i] = if c == '<' {true} else {false});

    let mut gas = gas.into_iter().enumerate().cycle();
    let mut rocks = [
        (4, 1, [1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0]),
        (3, 3, [0,1,0,0,1,1,1,0,0,1,0,0,0,0,0,0]),
        (3, 3, [1,1,1,0,0,0,1,0,0,0,1,0,0,0,0,0]),
        (1, 4, [1,0,0,0,1,0,0,0,1,0,0,0,1,0,0,0]),
        (2, 2, [1,1,0,0,1,1,0,0,0,0,0,0,0,0,0,0]),
    ].into_iter().cycle();
    let mut chamber = Chameber([0u8; WIDTH * HEIGHT]);
    let mut floor = 0;
    let mut record: SgMap<(usize, usize, usize), (usize, usize), 2000> = SgMap::new();
    let mut part1 = None;
    let mut part2 = None;

    for rock_index in 0..10000 {
        let (width, height, rock) = rocks.next().unwrap();
        let mut x = 2;
        let mut y = floor + 3;

        let gas_index = loop {
            let (i, is_left) = gas.next().unwrap();
            if is_left {
                if x > 0 && chamber.fit(&rock, x - 1, y) { x -= 1; }
            } else {
                if x + width < WIDTH && chamber.fit(&rock, x + 1, y) { x += 1; }
            }

            if y == 0 { break i; }
            if chamber.fit(&rock, x, y - 1) { y -= 1; } else { break i; }
        };
        chamber.place(&rock, x, y);
        floor = floor.max(y + height);

        if rock_index == 2022 { part1 = Some(floor); }

        if floor > 8 {
            let snapshot = (rock_index % 5, gas_index, chamber.snapshot(floor * WIDTH));
            if let Some((previous_index, previous_height)) = record.get(&snapshot) {
                let period = rock_index - previous_index;
                if MAX % period == rock_index % period {
                    let floor_period = floor - previous_height;
                    let num = (MAX - rock_index) / period;
                    part2 = Some(floor + num * floor_period - 1);
                    break;
                } 
            } else {
                record.insert(snapshot, (rock_index , floor));
            }
        }
    }

    println!("{part1:?}");
    println!("{part2:?}");
}

