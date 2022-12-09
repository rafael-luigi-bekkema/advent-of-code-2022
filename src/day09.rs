use std::collections::HashSet;

use crate::aoc::load_lines;

pub fn a() -> usize {
    _a(load_lines(9))
}

pub fn b() -> usize {
    _b(load_lines(9))
}

fn _a(moves: Vec<impl AsRef<str>>) -> usize {
    _ab(moves, 1)
}

fn _b(moves: Vec<impl AsRef<str>>) -> usize {
    _ab(moves, 9)
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

fn move_tail(head: &Pos, tail: &mut Pos) {
    if (head.x - tail.x).abs() <= 1 && (head.y - tail.y).abs() <= 1 {
        return;
    }
    if head.y != tail.y {
        tail.y += (head.y - tail.y) / (head.y - tail.y).abs();
    }
    if head.x != tail.x {
        tail.x += (head.x - tail.x) / (head.x - tail.x).abs();
    }
}

fn _ab(moves: Vec<impl AsRef<str>>, nr_tails: usize) -> usize {
    let mut head = Pos { x: 0, y: 0 };
    let mut tails = vec![Pos { x: 0, y: 0 }; nr_tails];
    let mut tail_poss = HashSet::new();

    for mov in moves.iter().map(|item| item.as_ref()) {
        let (dir, amount) = mov
            .split_once(' ')
            .map(|(s1, s2)| (s1, s2.parse::<i64>().unwrap()))
            .unwrap();

        for _i in 0..amount {
            match dir {
                "U" => head.y -= 1,
                "D" => head.y += 1,
                "L" => head.x -= 1,
                "R" => head.x += 1,
                _ => panic!("Unknown direction"),
            }

            let mut head2 = &head;
            for tail in tails.iter_mut() {
                move_tail(&head2, tail);

                head2 = tail;
            }

            tail_poss.insert(head2.clone());
        }
    }

    tail_poss.len()
}

#[cfg(test)]
mod tests {
    use crate::aoc::load_lines;

    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            13,
            _a(vec!["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2",])
        );
    }

    #[test]
    fn a2() {
        assert_eq!(6503, _a(load_lines(9)));
    }

    #[test]
    fn b() {
        assert_eq!(
            36,
            _b(vec![
                "R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20",
            ])
        );
    }

    #[test]
    fn b2() {
        assert_eq!(2724, _b(load_lines(9)));
    }
}
