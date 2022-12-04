use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn load_file() -> Vec<String> {
    let f = File::open("inputs/day04.txt").unwrap();

    BufReader::new(f).lines().map(|l| l.unwrap()).collect()
}

fn parse_range(range: &str) -> (u64, u64) {
    let (e1a, e1b) = range.split_once("-").unwrap();
    (e1a.parse::<u64>().unwrap(), e1b.parse::<u64>().unwrap())
}

fn _a(lines: Vec<String>) -> u64 {
    let mut count = 0u64;

    for line in lines.iter() {
        let (one, two) = line.split_once(",").unwrap();
        let (e1a, e1b) = parse_range(one);
        let (e2a, e2b) = parse_range(two);
        if e2a >= e1a && e2b <= e1b || e1a >= e2a && e1b <= e2b {
            count += 1;
        }
    }

    count
}

fn _b(lines: Vec<String>) -> u64 {
    let mut count = 0u64;

    for line in lines.iter() {
        let (one, two) = line.split_once(",").unwrap();
        let (e1a, e1b) = parse_range(one);
        let (e2a, e2b) = parse_range(two);
        if e1b < e2a || e1a > e2b {
            continue;
        }
        count += 1;
    }

    count
}

pub fn a() -> u64 {
    _a(load_file())
}

pub fn b() -> u64 {
    _b(load_file())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            2,
            _a(
                vec!["2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",]
                    .into_iter()
                    .map(|l| l.to_string())
                    .collect()
            )
        );
    }

    #[test]
    fn a2() {
        assert_eq!(431, _a(load_file()));
    }

    #[test]
    fn b() {
        assert_eq!(
            4,
            _b(
                vec!["2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",]
                    .into_iter()
                    .map(|l| l.to_string())
                    .collect()
            )
        );
    }

    #[test]
    fn b2() {
        assert_eq!(823, _b(load_file()));
    }
}
