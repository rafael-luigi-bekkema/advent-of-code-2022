use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn load_file() -> Vec<String> {
    let f = File::open("inputs/day03.txt").unwrap();

    BufReader::new(f).lines().map(|l| l.unwrap()).collect()
}

fn value(c: u8) -> u64 {
    (c - if c >= 97 {
        // 'a' - 'z'
        96
    } else {
        // 'A' - 'Z'
        38
    })
    .into()
}

pub fn a() -> u64 {
    _a(load_file())
}

fn _a(lines: Vec<String>) -> u64 {
    let mut total = 0u64;

    for line in lines.iter() {
        let chars: Vec<char> = line.chars().collect();
        let (compart1, compart2) = chars.split_at(chars.len() / 2);
        'outer: for c in compart1.iter() {
            for c2 in compart2.iter() {
                if *c == *c2 {
                    total += value(*c as u8);
                    break 'outer;
                }
            }
        }
    }

    total
}

pub fn b() -> u64 {
    _b(load_file())
}

fn _b(lines: Vec<String>) -> u64 {
    let mut total = 0u64;
    let mut idx = 0;
    while idx < lines.len() {
        'outer: for c in lines[idx].chars() {
            for c2 in lines[idx + 1].chars() {
                if c != c2 {
                    continue;
                }

                for c3 in lines[idx + 2].chars() {
                    if c == c3 {
                        total += value(c3 as u8);
                        break 'outer;
                    }
                }
            }
        }

        idx += 3;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            157,
            _a(vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            ]
            .into_iter()
            .map(|l| l.to_string())
            .collect())
        );
    }

    #[test]
    fn a2() {
        assert_eq!(8493, _a(load_file()));
    }

    #[test]
    fn b() {
        assert_eq!(
            70,
            _b(vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            ]
            .into_iter()
            .map(|l| l.to_string())
            .collect())
        );
    }

    #[test]
    fn b2() {
        assert_eq!(2552, _b(load_file()));
    }
}
