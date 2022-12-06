use std::collections::VecDeque;

use crate::aoc::load_text;

pub fn a() -> usize {
    _a(load_text(6))
}

pub fn b() -> usize {
    _b(load_text(6))
}

fn _a(line: impl AsRef<str>) -> usize {
    _ab(line.as_ref(), 4)
}

fn _b(line: impl AsRef<str>) -> usize {
    _ab(line.as_ref(), 14)
}

fn _ab(line: &str, msglen: usize) -> usize {
    let mut buf = VecDeque::new();

    'outer: for (i, char) in line.chars().enumerate() {
        buf.push_back(char);
        if i >= msglen {
            buf.pop_front().unwrap();
        }
        if i < msglen - 1 {
            continue;
        }
        for (j, c1) in buf.iter().take(msglen - 1).enumerate() {
            for c2 in buf.iter().skip(j + 1) {
                if c1 == c2 {
                    continue 'outer;
                }
            }
        }
        return i + 1;
    }

    panic!("solution not found!");
}

#[cfg(test)]
mod tests {
    use crate::aoc::load_text;

    use super::*;

    #[test]
    fn a() {
        assert_eq!(7, _a("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    }

    #[test]
    fn a2() {
        assert_eq!(5, _a("bvwbjplbgvbhsrlpgdmjqwftvncz"));
    }

    #[test]
    fn a3() {
        assert_eq!(6, _a("nppdvjthqldpwncqszvftbrmjlhg"));
    }

    #[test]
    fn a4() {
        assert_eq!(10, _a("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    }

    #[test]
    fn a5() {
        assert_eq!(11, _a("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn a6() {
        assert_eq!(1282, _a(load_text(6)));
    }

    #[test]
    fn b() {
        assert_eq!(19, _b("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    }

    #[test]
    fn b2() {
        assert_eq!(23, _b("bvwbjplbgvbhsrlpgdmjqwftvncz"));
    }

    #[test]
    fn b3() {
        assert_eq!(23, _b("nppdvjthqldpwncqszvftbrmjlhg"));
    }

    #[test]
    fn b4() {
        assert_eq!(29, _b("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    }

    #[test]
    fn b5() {
        assert_eq!(26, _b("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn b6() {
        assert_eq!(3513, _b(load_text(6)));
    }
}
