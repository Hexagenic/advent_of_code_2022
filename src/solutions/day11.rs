use crate::solutions::Solution;

#[derive(PartialEq, Eq, Debug)]
enum MonkeyOperation {
    Add(i64),
    AddOld,
    Multiply(i64),
    MultiplyOld,
}

#[derive(PartialEq, Eq, Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: MonkeyOperation,
    test: i64,
    true_monkey: usize,
    false_monkey: usize,
}

fn parse_monkeys(file: &str) -> impl Iterator<Item = Monkey> + '_ {
    file.split("\n\n").map(parse_monkey)
}

fn parse_monkey(spec: &str) -> Monkey {
    let mut lines = spec.lines();
    lines.next();

    let items = lines
        .next()
        .unwrap()
        .split_at(18)
        .1
        .split(", ")
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let operation = parse_op(lines.next().unwrap().split_at(19).1);

    let test = lines.next().unwrap().split_at(21).1.parse().unwrap();

    let true_monkey = lines.next().unwrap().split_at(29).1.parse().unwrap();
    let false_monkey = lines.next().unwrap().split_at(30).1.parse().unwrap();

    Monkey {
        items,
        operation,
        test,
        true_monkey,
        false_monkey,
    }
}

fn parse_op(spec: &str) -> MonkeyOperation {
    let mut parts = spec.split(' ');
    let op = parts.nth(1).unwrap().chars().next().unwrap();
    let value = parts.next().unwrap();

    match (op, value) {
        ('*', "old") => MonkeyOperation::MultiplyOld,
        ('+', "old") => MonkeyOperation::AddOld,
        ('*', v) => MonkeyOperation::Multiply(v.parse().unwrap()),
        ('+', v) => MonkeyOperation::Add(v.parse().unwrap()),
        _ => panic!("Unknown monkey operation"),
    }
}

fn update_worry(worry: i64, op: &MonkeyOperation) -> i64 {
    match op {
        MonkeyOperation::AddOld => worry + worry,
        MonkeyOperation::Add(v) => worry + v,
        MonkeyOperation::MultiplyOld => worry * worry,
        MonkeyOperation::Multiply(v) => worry * v,
    }
}

fn monkey_business<W>(monkeys: &mut [Monkey], rounds: i32, manage_worry: W) -> i64
where
    W: Fn(i64) -> i64,
{
    let mut inspection: Vec<i64> = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            for worry in monkeys[m].items.clone() {
                let new_worry = manage_worry(update_worry(worry, &monkeys[m].operation));

                let next_monkey = if new_worry % monkeys[m].test == 0 {
                    monkeys[m].true_monkey
                } else {
                    monkeys[m].false_monkey
                };

                monkeys[next_monkey].items.push(new_worry);
                inspection[m] += 1;
            }
            monkeys[m].items.clear();
        }
    }

    inspection.sort_by(|a, b| b.cmp(a));
    inspection.iter().take(2).product()
}

pub fn part_a(file: &str) -> Solution {
    let mut monkeys = parse_monkeys(file).collect::<Vec<_>>();
    Solution::Integer(monkey_business(&mut monkeys, 20, |w| w / 3))
}

pub fn part_b(file: &str) -> Solution {
    let mut monkeys = parse_monkeys(file).collect::<Vec<_>>();
    let test_product: i64 = monkeys.iter().map(|m| m.test).product::<i64>();
    Solution::Integer(monkey_business(&mut monkeys, 10000, |w| w % test_product))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_test_data() -> String {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day11_test.txt");
        std::fs::read_to_string(&path).unwrap()
    }

    #[test]
    fn test_parse() {
        let monkeys = parse_monkeys(&read_test_data()).collect::<Vec<_>>();

        assert_eq!(4, monkeys.len());

        assert_eq!(vec![54, 65, 75, 74], monkeys[1].items);
        assert_eq!(MonkeyOperation::Add(3), monkeys[3].operation);

        assert_eq!(
            Monkey {
                items: vec![79, 60, 97],
                operation: MonkeyOperation::MultiplyOld,
                test: 13,
                true_monkey: 1,
                false_monkey: 3
            },
            monkeys[2]
        );
    }

    #[test]
    fn test_part_a() {
        assert_eq!(Solution::Integer(10605), part_a(&read_test_data()));
    }

    #[test]
    fn test_monkey_business_part2() {
        assert_eq!(Solution::Integer(2_713_310_158), part_b(&read_test_data()));
    }
}
