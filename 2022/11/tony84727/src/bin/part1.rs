use tony84727::parse_monkeys;

fn main() {
    let monkeys = parse_monkeys(std::io::stdin().lines().map(|line| line.unwrap()));
    let mut items = monkeys
        .iter()
        .map(|m| m.items.clone())
        .collect::<Vec<Vec<i32>>>();
    let mut monkey_inspect = vec![0; monkeys.len()];
    for _round in 0..20 {
        for (m, monkey) in monkeys.iter().enumerate() {
            let monkey_items = std::mem::replace(&mut items[m], vec![]);
            monkey_inspect[m] += monkey_items.len();
            for item in monkey_items.iter() {
                let mut item = *item;
                item = monkey.operation.calculate(item);
                item /= 3;
                if item % monkey.test == 0 {
                    items[monkey.true_to as usize].push(item);
                } else {
                    items[monkey.false_to as usize].push(item);
                }
            }
        }
    }
    monkey_inspect.sort_unstable();

    println!(
        "{}",
        monkey_inspect
            .into_iter()
            .rev()
            .take(2)
            .fold(1, |a, b| a * b)
    );
}
