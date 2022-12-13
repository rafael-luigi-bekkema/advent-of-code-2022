use std::{cmp::Ordering, collections::VecDeque, str};

use crate::aoc::load_lines;

pub fn a() -> usize {
    _a(load_lines(13))
}

#[derive(Debug)]
enum Item {
    List(Vec<Item>),
    Value(usize),
}

fn parse_list(line: &mut VecDeque<u8>) -> Vec<Item> {
    let mut list = Vec::new();

    if line[0] != b'[' {
        panic!("expected list, got something else");
    }
    line.pop_front();

    loop {
        if line.is_empty() {
            break;
        }

        if line[0] == b'[' {
            let l2 = parse_list(line);
            list.push(Item::List(l2));
            continue;
        }

        let idx = line
            .iter()
            .position(|item| *item == b',' || *item == b']')
            .unwrap();
        if idx > 0 {
            let bit: Vec<u8> = line.iter().take(idx).copied().collect();
            let num = str::from_utf8(&bit).unwrap().parse::<usize>().unwrap();
            list.push(Item::Value(num));

            for _ in 0..idx + 1 {
                line.pop_front();
            }
        }

        if !line.is_empty() && (line[0] == b',' || line[0] == b']') {
            line.pop_front();
            break;
        }
    }

    list
}

fn cmp_item(i1: &Item, i2: &Item) -> Ordering {
    match (i1, i2) {
        (Item::Value(v1), Item::Value(v2)) => v1.cmp(v2),
        (Item::List(l1), Item::Value(v2)) => cmp(l1, &vec![Item::Value(*v2)]),
        (Item::Value(v1), Item::List(l2)) => cmp(&vec![Item::Value(*v1)], l2),
        (Item::List(l1), Item::List(l2)) => cmp(l1, l2),
    }
}

fn cmp(i1: &Vec<Item>, i2: &Vec<Item>) -> Ordering {
    for (i, item) in i1.iter().enumerate() {
        if i >= i2.len() {
            return Ordering::Greater;
        }
        let r = cmp_item(item, &i2[i]);
        if r != Ordering::Equal {
            return r;
        }
    }
    if i1.len() < i2.len() {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn _a(lines: Vec<String>) -> usize {
    let mut count = 1usize;
    let mut correct = Vec::new();
    for (i, line) in lines.iter().enumerate().step_by(3) {
        let s1 = parse_list(&mut line.bytes().collect());
        let s2 = parse_list(&mut lines[i + 1].bytes().collect());

        if cmp(&s1, &s2) != Ordering::Greater {
            correct.push(count);
        }

        count += 1;
    }

    correct.iter().sum()
}

fn _b(mut lines: Vec<String>) -> usize {
    lines.push(String::new());
    lines.push("[[2]]".to_string());
    lines.push("[[6]]".to_string());

    let mut items = Vec::new();
    for (i, line) in lines.iter().enumerate().step_by(3) {
        let s1 = parse_list(&mut line.bytes().collect());
        let s2 = parse_list(&mut lines[i + 1].bytes().collect());

        items.push(Item::List(s1));
        items.push(Item::List(s2));
    }

    items.sort_by(|i1, i2| cmp_item(i1, i2));

    let d1 = "List([List([Value(2)])])";
    let d2 = "List([List([Value(6)])])";

    let pos1 = items.iter().position(|item| format!("{item:?}") == d1).unwrap() + 1;
    let pos2 = items.iter().position(|item| format!("{item:?}") == d2).unwrap() + 1;

    pos1 * pos2
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc::load_lines_suffix;

    #[test]
    fn a() {
        assert_eq!(13, _a(load_lines_suffix(13, "_test")));
    }

    #[test]
    fn a2() {
        assert_eq!(5185, _a(load_lines(13)));
    }

    #[test]
    fn b() {
        assert_eq!(140, _b(load_lines_suffix(13, "_test")));
    }

    #[test]
    fn b2() {
        assert_eq!(23751, _b(load_lines(13)));
    }
}
