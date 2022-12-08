use crate::aoc::load_lines;

pub fn a() -> u64 {
    _a(load_lines(1))
}

pub fn b() -> u64 {
    _b(load_lines(1))
}

fn parse_lines(lines: &[String]) -> Vec<u64> {
    let mut counts = Vec::from([0u64]);

    for line in lines.iter() {
        if line.is_empty() {
            counts.push(0);
            continue;
        }
        counts.last_mut().map(|v| *v += line.parse::<u64>().unwrap());
    }

    counts
}

fn _a(lines: Vec<String>) -> u64 {
    parse_lines(&lines).into_iter().max().unwrap()
}

fn _b(lines: Vec<String>) -> u64 {
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
        assert_eq!(71124, _a(load_lines(1)));
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
        assert_eq!(204639, _b(load_lines(1)));
    }
}
