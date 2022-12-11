use crate::aoc::load_lines;

pub fn a() -> u64 {
    _a(load_lines(1))
}

pub fn b() -> u64 {
    _b(load_lines(1))
}

fn parse_lines(lines: &[impl AsRef<str>]) -> Vec<u64> {
    let mut counts = Vec::from([0u64]);

    for line in lines.iter().map(|l| l.as_ref()) {
        if line.is_empty() {
            counts.push(0);
            continue;
        }
        *counts.last_mut().unwrap() += line.parse::<u64>().unwrap();
    }

    counts
}

fn _a(lines: Vec<impl AsRef<str>>) -> u64 {
    parse_lines(&lines).into_iter().max().unwrap()
}

fn _b(lines: Vec<impl AsRef<str>>) -> u64 {
    let mut counts = parse_lines(&lines);

    counts.sort();
    counts.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            24000,
            _a(vec![
                "1000",
                "2000",
                "3000",
                "",
                "4000",
                "",
                "5000",
                "6000",
                "",
                "7000",
                "8000",
                "9000",
                "",
                "10000",
            ])
        );
    }

    #[test]
    fn a2() {
        assert_eq!(71124, _a(load_lines(1)));
    }

    #[test]
    fn b() {
        assert_eq!(
            45000,
            _b(vec![
                "1000",
                "2000",
                "3000",
                "",
                "4000",
                "",
                "5000",
                "6000",
                "",
                "7000",
                "8000",
                "9000",
                "",
                "10000",
            ])
        );
    }

    #[test]
    fn b2() {
        assert_eq!(204639, _b(load_lines(1)));
    }
}
