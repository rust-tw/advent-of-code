mod valve;

use std::collections::{BinaryHeap, HashMap};
use valve::{create_routes, Valve};

type States = HashMap<String, Vec<Vec<Option<u64>>>>;

pub fn challenge_16(lines: Vec<&str>) -> (u64, u64) {
    let (solver, mut states) = Solver::new(lines);
    let visitables = solver.get_visitables();
    let part1 = solver.find_max_pressure(30, visitables, &mut states);
    let part2 = (0..((visitables + 1) / 2))
        .map(|x| {
            solver.find_max_pressure(26, x, &mut states)
                + solver.find_max_pressure(26, visitables - x, &mut states)
        })
        .max()
        .unwrap();

    (part1, part2)
}

struct Solver {
    valves: HashMap<String, Valve>,
    path_map: HashMap<String, HashMap<String, usize>>,
    weighted_valves: Vec<String>,
}

impl Solver {
    pub fn new(lines: Vec<&str>) -> (Self, States) {
        let valves = create_routes(lines).unwrap();

        let path_map = valves
            .iter()
            .map(|(k, v)| {
                let path = if k == &String::from("AA") || v.flow_rate != 0 {
                    Self::create_shortest_paths(&valves, k)
                } else {
                    HashMap::new()
                };
                (k.clone(), path)
            })
            .collect::<HashMap<_, _>>();

        let weighted_valves = valves
            .iter()
            .filter(|(_, v)| v.flow_rate != 0)
            .map(|(k, _)| k.clone())
            .collect::<Vec<_>>();

        let visit_possibles = 1 << weighted_valves.len();
        let states = weighted_valves
            .iter()
            .map(|name| (name.clone(), vec![vec![None; visit_possibles]; 30]))
            .collect::<HashMap<_, _>>();

        (
            Solver {
                valves,
                path_map,
                weighted_valves,
            },
            states,
        )
    }

    fn get_visitables(&self) -> u64 {
        (1 << self.weighted_valves.len()) - 1
    }

    fn create_shortest_paths(
        routes: &HashMap<String, Valve>,
        start: &String,
    ) -> HashMap<String, usize> {
        let mut distances: HashMap<_, _> = routes
            .iter()
            .map(|(k, _)| (k.clone(), usize::MAX))
            .collect();
        distances
            .entry(start.clone())
            .and_modify(|x| *x = 0)
            .or_insert(0);

        let mut q = BinaryHeap::new();
        q.push((start, 0));

        while let Some((position, cost)) = q.pop() {
            if cost > distances[position] {
                continue;
            }

            for next_pos in &routes[position].leads_to {
                let next_cost = cost + 1;
                if next_cost < distances[next_pos] {
                    distances
                        .entry(next_pos.clone())
                        .and_modify(|x| *x = next_cost)
                        .or_insert(next_cost);
                    q.push((next_pos, next_cost))
                }
            }
        }

        distances
    }

    pub fn find_max_pressure(&self, time_limit: u64, visitables: u64, states: &mut States) -> u64 {
        let candidates: Vec<_> = self
            .weighted_valves
            .iter()
            .enumerate()
            .map(|(idx, name)| {
                if visitables & (1 << idx) != 0 {
                    let unvisited_list = visitables & !(1 << idx);
                    self.find_max(
                        name,
                        time_limit - self.path_map[&String::from("AA")][name] as u64,
                        unvisited_list,
                        states,
                    )
                } else {
                    0
                }
            })
            .collect();
        candidates.into_iter().max().unwrap_or(0)
    }

    fn find_max(&self, name: &String, time_left: u64, unvisited: u64, states: &mut States) -> u64 {
        match states[name][time_left as usize][unvisited as usize] {
            Some(value) => value,
            None => {
                let max_child = self
                    .weighted_valves
                    .iter()
                    .enumerate()
                    .map(|(idx, next)| {
                        if (1 << idx & unvisited) != 0
                            && (time_left - 1) > (self.path_map[name][next] as u64)
                        {
                            self.find_max(
                                next,
                                time_left - 1 - (self.path_map[name][next] as u64),
                                unvisited & !(1 << idx),
                                states,
                            )
                        } else {
                            0
                        }
                    })
                    .max()
                    .unwrap_or(0);

                let new_max = max_child + self.valves[name].flow_rate * (time_left - 1);
                states.entry(name.clone()).and_modify(|v| {
                    v[time_left as usize][unvisited as usize].replace(new_max);
                });
                new_max
            }
        }
    }
}
