use std::collections::VecDeque;

use crate::aoc::load_lines;

pub fn a() -> u64 {
    _a(load_lines(11))
}

pub fn b() -> u64 {
    _b(load_lines(11))
}

#[derive(Clone, Debug)]
enum Op {
    Add,
    Mul,
}

#[derive(Clone, Debug)]
enum On {
    Old,
    Val(u64),
}

#[derive(Clone, Debug)]
struct Test {
    div: u64,
    if_true: usize,
    if_false: usize,
}

#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<u64>,
    on: On,
    op: Op,
    test: Test,
    inspects: u64,
}

fn parse_monkeys(input: Vec<&str>) -> Vec<Monkey> {
    let mut monkeys = vec![];
    let mut monkey = Monkey {
        items: VecDeque::new(),
        on: On::Old,
        op: Op::Add,
        test: Test {
            div: 0,
            if_true: 0,
            if_false: 0,
        },
        inspects: 0,
    };

    for line in input {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("  Starting items: ") {
            let items: String = line.chars().skip(18).collect();
            for item in items.split(", ").map(|v| v.parse::<u64>().unwrap()) {
                monkey.items.push_back(item);
            }
            continue;
        }
        if line.starts_with("  Operation: new = ") {
            let sop: String = line.chars().skip(19).collect();
            let op: Vec<&str> = sop.split(' ').collect();
            match op[1] {
                "*" => monkey.op = Op::Mul,
                "+" => monkey.op = Op::Add,
                _ => panic!("unknown op: {}", op[1]),
            }
            match op[2] {
                "old" => monkey.on = On::Old,
                val => monkey.on = On::Val(val.parse().unwrap()),
            }
            continue;
        }
        if line.starts_with("  Test: divisible by ") {
            monkey.test.div = line.chars().skip(21).collect::<String>().parse().unwrap();
            continue;
        }
        if line.starts_with("    If true: throw to monkey ") {
            monkey.test.if_true = line.chars().skip(29).collect::<String>().parse().unwrap();
            continue;
        }
        if line.starts_with("    If false: throw to monkey ") {
            monkey.test.if_false = line.chars().skip(30).collect::<String>().parse().unwrap();

            monkeys.push(monkey.clone());
            monkey.items = VecDeque::new();
            continue;
        }
    }

    monkeys
}

fn _a(input: Vec<impl AsRef<str>>) -> u64 {
    let mut monkeys = parse_monkeys(input.iter().map(|v| v.as_ref()).collect());

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys[i].clone();
            monkeys[i].items.clear();

            for item in monkey.items.drain(..) {
                let with = match monkey.on {
                    On::Old => item,
                    On::Val(val) => val,
                };
                let newval = match monkey.op {
                    Op::Mul => with * item,
                    Op::Add => with + item,
                } / 3;
                let to = if newval % monkey.test.div == 0 {
                    monkey.test.if_true
                } else {
                    monkey.test.if_false
                };

                monkeys[i].inspects += 1;
                monkeys[to].items.push_back(newval);
            }
        }
    }

    let mut inspects: Vec<u64> = monkeys.into_iter().map(|m| m.inspects).collect();
    inspects.sort();
    inspects.reverse();

    inspects[0] * inspects[1]
}

fn lcm(nums: &mut [u64]) -> u64 {
    nums.sort();
    let mut base = nums[0];
    'outer: loop {
        for item in nums.iter().skip(1) {
            if (base % *item) != 0 {
                base += nums[0];
                continue 'outer;
            }
        }
        break;
    }
    base
}

fn _b(input: Vec<impl AsRef<str>>) -> u64 {
    let mut monkeys = parse_monkeys(input.iter().map(|v| v.as_ref()).collect());

    let common_div = lcm(&mut monkeys.iter().map(|m| m.test.div).collect::<Vec<u64>>());

    for _round in 0..10_000 {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys[i].clone();
            monkeys[i].items.clear();

            for item in monkey.items.drain(..) {
                let with = match monkey.on {
                    On::Old => item,
                    On::Val(val) => val,
                };
                let newval = match monkey.op {
                    Op::Mul => with * item,
                    Op::Add => with + item,
                };
                let to = if newval % monkey.test.div == 0 {
                    monkey.test.if_true
                } else {
                    monkey.test.if_false
                };

                monkeys[i].inspects += 1;
                monkeys[to].items.push_back(newval % common_div);
            }
        }
    }

    let mut inspects: Vec<u64> = monkeys.into_iter().map(|m| m.inspects).collect();
    inspects.sort();
    inspects.reverse();

    inspects[0] * inspects[1]
}

#[cfg(test)]
mod tests {
    use crate::aoc::load_lines_suffix;

    use super::*;

    #[test]
    fn test_lcm() {
        assert_eq!(96_577, lcm(&mut [23, 19, 13, 17]))
    }

    #[test]
    fn a() {
        assert_eq!(10605, _a(load_lines_suffix(11, "_test")));
    }

    #[test]
    fn a2() {
        assert_eq!(108240, _a(load_lines(11)));
    }

    #[test]
    fn b() {
        assert_eq!(2713310158, _b(load_lines_suffix(11, "_test")));
    }

    #[test]
    fn b2() {
        assert_eq!(25712998901, _b(load_lines(11)));
    }
}
