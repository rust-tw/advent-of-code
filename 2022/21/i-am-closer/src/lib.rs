use std::collections::HashMap;

pub fn challenge_21(lines: Vec<&str>) -> (i64, i64) {
    let mut map: HashMap<&str, Monkey> =
        HashMap::from_iter(lines.iter().map(|&s| Monkey::from_str(s)));

    let part1 = if let Monkey::Value(Num(n)) = map["humn"] {
        eval(&map, "root", n)
    } else {
        0
    };

    let part2 = if let Monkey::DoMath(_, Str(label0), Str(label1)) = map["root"] {
        // Cache all names that can be evaluated
        construct_map(&mut map, label0);
        construct_map(&mut map, label1);

        // Bisection method
        const DIVIDER: i64 = 32;            // prevent overflow
        let mut max = i64::MAX / DIVIDER;
        let mut min = i64::MIN / DIVIDER;
        let mut mid = max / 2;
        loop {
            let comp_max = eval(&map, label0, max).cmp(&eval(&map, label1, max));
            let comp_mid = eval(&map, label0, mid).cmp(&eval(&map, label1, mid));
            let comp_min = eval(&map, label0, min).cmp(&eval(&map, label1, min));
            if comp_mid.is_eq() {
                break mid;
            }

            if comp_max == comp_mid {
                max = mid;
                mid = min + (mid - min) / 2;
            } else if comp_min == comp_mid {
                min = mid;
                mid = mid + (max - mid) / 2;
            } else {
                panic!("ERROR 1");
            }

            if max == min {
                panic!("ERROR 2");
            }
        }
    } else {
        0
    };

    (part1, part2)
}

use Value::*;

fn construct_map<'a>(
    map: &mut HashMap<&'a str, Monkey<'a>>,
    name: &'a str,
) -> Monkey<'a> {
    if name == "humn" {
        return Monkey::Value(Str("humn"));
    }
    match *map.get(name).expect(&format!("{}!!!", name)) {
        Monkey::Value(v) => Monkey::Value(v),
        Monkey::DoMath(op, left, right) => {
            let left = match left {
                Str(s) => construct_map(map, s),
                n @ Num(_) => Monkey::Value(n),
            };
            let right = match right {
                Str(s) => construct_map(map, s),
                n @ Num(_) => Monkey::Value(n),
            };
            match (left, right) {
                (Monkey::Value(Num(left)), Monkey::Value(Num(right))) => {
                    let value = match op {
                        "+" => left + right,
                        "-" => left - right,
                        "*" => left * right,
                        "/" => left / right,
                        _ => 0,
                    };
                    *(map.get_mut(name).unwrap()) = Monkey::Value(Num(value));
                    Monkey::Value(Num(value))
                }
                (Monkey::Value(left), Monkey::Value(right)) => {
                    *(map.get_mut(name).unwrap()) = Monkey::DoMath(op, left, right);
                    Monkey::Value(Str(name))
                }
                _ => panic!("Should not be DoMath..."),
            }
        }
    }
}

fn eval<'a>(map: &HashMap<&'a str, Monkey<'a>>, name: &'a str, human: i64) -> i64 {
    if name == "humn" {
        return human;
    }
    match *map.get(name).expect(&format!("{}!!!", name)) {
        Monkey::Value(Num(v)) => v,
        Monkey::DoMath(op, left, right) => {
            let left = match left {
                Str(s) => eval(map, s, human),
                Num(n) => n,
            };
            let right = match right {
                Str(s) => eval(map, s, human),
                Num(n) => n,
            };
            let value = match op {
                "+" => left + right,
                "-" => left - right,
                "*" => left * right,
                "/" => left / right,
                _ => 0,
            };
            value
        }
        _ => panic!("Str node encountered. Cannot be resolved!"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Monkey<'a> {
    Value(Value<'a>),
    DoMath(&'a str, Value<'a>, Value<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Value<'a> {
    Num(i64),
    Str(&'a str),
}

impl<'a> Monkey<'a> {
    pub fn from_str(s: &'a str) -> (&'a str, Monkey) {
        let [name, rest]: [&str; 2] = s.split(':').collect::<Vec<_>>().try_into().unwrap();
        let tokens = rest.trim().split(' ').collect::<Vec<_>>();
        let monkey = if tokens.len() == 1 {
            Monkey::Value(Value::Num(tokens[0].parse::<i64>().unwrap()))
        } else {
            Monkey::DoMath(tokens[1], Value::Str(tokens[0]), Value::Str(tokens[2]))
        };
        (name, monkey)
    }
}
