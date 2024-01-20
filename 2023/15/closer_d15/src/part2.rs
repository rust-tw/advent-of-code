use closer_d15::*;

pub fn solve(line: &str) -> u64 {
    let mut boxes: Vec<Vec<(String, u64)>> = vec![vec![]; 256];
    line.split(',').for_each(|s| {
        let op = s.parse::<Op>().unwrap();
        let box_idx = hashing(op.label());
        let pos = boxes[box_idx]
            .iter()
            .enumerate()
            .find(|(_, (s, _))| *s == op.label())
            .map(|(slot_idx, _)| slot_idx);

        if let Some(slot_idx) = pos {
            match &op {
                Op::Update(s, focal_len) => boxes[box_idx][slot_idx] = (s.clone(), *focal_len),
                Op::Remove(_) => {
                    boxes[box_idx].remove(slot_idx);
                }
            }
        } else {
            match &op {
                Op::Update(s, focal_len) => boxes[box_idx].push((s.clone(), *focal_len)),
                _ => (),
            }
        }
    });

    boxes
        .into_iter()
        .enumerate()
        .map(|(box_idx, slots)| {
            slots
                .into_iter()
                .enumerate()
                .map(|(slot_idx, (_, focal_len))| {
                    (box_idx as u64 + 1) * (slot_idx as u64 + 1) * focal_len
                })
                .sum::<u64>()
        })
        .sum()
}
