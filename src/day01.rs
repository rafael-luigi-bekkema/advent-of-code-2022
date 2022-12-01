use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::HashMap,
};

fn load_from_file() -> Vec<String> {
    let f = File::open("inputs/day01.txt").unwrap();
    BufReader::new(f)
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

pub fn a() -> u64 {
    _a(load_from_file())
}

pub fn b() -> u64 {
    _b(load_from_file())
}

fn _a(lines: Vec<String>) -> u64 {
    let mut counts = HashMap::new();

    let mut elf = 1u64;
    for line in lines.iter() {
        if line == "" {
            elf += 1;
            continue;
        }
        let amount = line.parse::<u64>().unwrap();
        let item = counts.entry(elf).or_insert(0);
        *item += amount;
    }

    let mut max_count = 0u64;
    for (_elf, count) in counts {
        if count > max_count {
            max_count = count;
        }
    }

    max_count
}

fn _b(lines: Vec<String>) -> u64 {
    let mut counts = HashMap::new();

    let mut elf = 1u64;
    for line in lines.iter() {
        if line == "" {
            elf += 1;
            continue;
        }
        let amount = line.parse::<u64>().unwrap();
        let item = counts.entry(elf).or_insert(0);
        *item += amount;
    }

    let mut counts: Vec<u64> = counts.into_values().collect();
    counts.sort();
    counts.reverse();

    counts[0..3].into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            24000,
            _a(vec![
                "1000".to_string(),
                "2000".to_string(),
                "3000".to_string(),
                "".to_string(),
                "4000".to_string(),
                "".to_string(),
                "5000".to_string(),
                "6000".to_string(),
                "".to_string(),
                "7000".to_string(),
                "8000".to_string(),
                "9000".to_string(),
                "".to_string(),
                "10000".to_string(),
            ])
        );
    }

    #[test]
    fn a2() {
        assert_eq!(
            71124,
            _a(load_from_file())
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            45000,
            _b(vec![
                "1000".to_string(),
                "2000".to_string(),
                "3000".to_string(),
                "".to_string(),
                "4000".to_string(),
                "".to_string(),
                "5000".to_string(),
                "6000".to_string(),
                "".to_string(),
                "7000".to_string(),
                "8000".to_string(),
                "9000".to_string(),
                "".to_string(),
                "10000".to_string(),
            ])
        );
    }

    #[test]
    fn b2() {
        assert_eq!(
            204639,
            _b(load_from_file())
        );
    }
}
