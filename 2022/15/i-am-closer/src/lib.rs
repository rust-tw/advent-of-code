mod sensor;
use rayon::prelude::*;
use sensor::*;
use std::collections::HashSet;
use std::time::Instant;

pub fn challenge_15(lines: Vec<&str>) -> (usize, u64) {
    let sensors = lines
        .into_iter()
        .map(|line| line.parse::<Sensor>().unwrap())
        .collect::<Vec<_>>();

    let beacons: HashSet<_> = sensors.iter().map(|sensor| sensor.get_beacon()).collect();

    let (y, search_range) = if sensors[0].get_beacon().x.abs() < 100 {
        // input_short.txt
        (10, 20)
    } else {
        // input.txt
        (2000000, 4000000)
    };

    //--- The following lines look more elegant, but much slower...
    // let occupied_grids = sensors
    //     .iter()
    //     .flat_map(|x| x.scanned_blocks(y))
    //     .collect::<HashSet<_>>()
    //     .len();

    let mut ranges = sensors
        .iter()
        .map(|sensor| sensor.range_in_row(y))
        .filter(|r| r.is_some())
        .collect::<Option<Vec<_>>>()
        .unwrap();
    ranges.sort_by(|(start1, _), (start2, _)| start1.cmp(start2));

    let (occupied_grids, _) =
        ranges
            .iter()
            .fold((0usize, i32::MIN), |(accu, last), &(start, end)| {
                if start <= last {
                    if end <= last {
                        (accu, last)
                    } else {
                        (accu + (end - last) as usize, end)
                    }
                } else {
                    (accu + (end - start + 1) as usize, end)
                }
            });

    let beacons_in_row = beacons
        .iter()
        .filter(|Beacon { y: beacon_y, .. }| *beacon_y == y)
        .count();

    let part1 = occupied_grids - beacons_in_row;

    let start_time = Instant::now();
    let part2 = (0..=search_range)
        .into_par_iter() // boost with parellel computation!!!
        .map(|y| {
            let mut ranges = sensors
                .iter()
                .filter_map(|sensor| sensor.range_in_row(y))
                .filter(|&(start, end)| start <= search_range && end >= 0)
                .map(|(start, end)| (start.max(0), (end.min(search_range))))
                .collect::<Vec<_>>();
            ranges.sort_by(|(start1, _), (start2, _)| start1.cmp(start2));

            let r = ranges
                .into_iter()
                .reduce(|(a_start, a_end), (start, end)| {
                    if start <= a_end + 1 {
                        if end <= a_end {
                            (a_start, a_end)
                        } else {
                            (a_start, end)
                        }
                    } else {
                        (a_start, a_end)
                    }
                })
                .unwrap();

            if r.1 != search_range {
                Some(((r.1 + 1) as u64) * 4000000 + (y as u64))
            } else if r.0 != 0 {
                Some(y as u64)
            } else {
                None
            }
        })
        .filter(|x| x.is_some())
        .collect::<Option<Vec<_>>>()
        .unwrap()[0];

    let duration = start_time.elapsed();
    println!("Duration: {:?}", duration);

    (part1, part2)
}
