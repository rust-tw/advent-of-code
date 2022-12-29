mod blueprint;

use blueprint::Blueprint;
use std::collections::VecDeque;

pub fn challenge_19(lines: Vec<&str>) -> (u64, u64) {
    let bps = lines
        .iter()
        .map(|line| line.parse::<Blueprint>().unwrap())
        .collect::<Vec<_>>();

    let part1 = bps
        .iter()
        .enumerate()
        .map(|(idx, bp)| eval_blueprint(bp, 24) * (idx as u64 + 1))
        .sum();

    let part2 = bps
        .iter()
        .take(3)
        .map(|bp| eval_blueprint(bp, 32))
        .product();

    (part1, part2)
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct State {
    time_left: u64,
    materials: [u64; 4],
    robots: [u64; 4],
}

fn eval_blueprint(bp: &Blueprint, time_limit: u64) -> u64 {
    let mut q = VecDeque::new();

    q.push_back(State {
        time_left: time_limit,
        materials: [0, 0, 0, 0],
        robots: [1, 0, 0, 0],
    });

    // Robot factory 只有一個，每一分鐘只能生產一個機器人。
    // 所以每一種機器人的數量只要大過每分鐘最多可消耗的材料，就不需要再生產了。
    let max_robots = (0..4)
        .map(|idx| {
            if idx == 3 {
                u64::MAX
            } else {
                bp.recipes.iter().map(|cost| cost[idx]).max().unwrap()
            }
        })
        .collect::<Vec<_>>();

    let mut max_geode = 0;

    while let Some(State {
        time_left,
        materials,
        robots,
    }) = q.pop_front()
    {
        // 針對每一種機器人，檢查還需要多久才能生出下一台
        for robot_type in 0..4 {
            // 檢查 機器人的數量是否大過每分鐘最多可消耗的材料
            if robots[robot_type] >= max_robots[robot_type] {
                continue;
            }

            let recipe = bp.recipes[robot_type];
            let time_required = (0..4)
                .map(|idx| {
                    match recipe[idx] {
                        // 如果所需原料足夠，那麼當下就可以生產。
                        cost if cost <= materials[idx] => 0,
                        // 若是沒有生產所需原料的機器人，那麼等再久都沒有用。
                        _ if robots[idx] == 0 => u64::MAX - 1,
                        // 不足的原料，還需要多久才能生產出來？
                        // 分子部份「+ 分母 - 1」，是在做「無條件進位法」。
                        cost => ((cost - materials[idx]) + robots[idx] - 1) / robots[idx],
                    }
                })
                .max()
                .unwrap();

            if time_required + 1 >= time_left {
                continue;
            }

            // 在等待原料 ready、生出新的 robot 的時間中，可以蒐集到多少原料
            let new_materials: [u64; 4] = (0..4)
                .map(|idx| materials[idx] + robots[idx] * (time_required + 1) - recipe[idx])
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            let new_robots: [u64; 4] = (0..4)
                .map(|idx| {
                    if idx == robot_type {
                        robots[idx] + 1
                    } else {
                        robots[idx]
                    }
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            let new_time_left = time_left - time_required - 1;

            // 如果剩下的時間裡，每一分鐘都產生一台新的 geode robot，
            // 那最後產量會超過現在的 max_geode 嗎？不會的話就不用玩了。
            if new_materials[3]
                + new_time_left * new_robots[3]
                + (new_time_left - 1) * new_time_left / 2   // 等差級數公式
                < max_geode
            {
                continue;
            }

            let new_state = State {
                time_left: new_time_left,
                materials: new_materials,
                robots: new_robots,
            };

            q.push_back(new_state);
        }

        max_geode = max_geode.max(materials[3] + robots[3] * time_left);
    }
    max_geode
}
