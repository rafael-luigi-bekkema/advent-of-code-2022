use crate::aoc::load_lines;

pub fn a() -> i64 {
    _a(load_lines(10))
}

pub fn b() -> String {
    _b(load_lines(10), ' ')
}

fn _a(lines: Vec<String>) -> i64 {
    let mut reg = 1i64;
    let mut cycle = 1i64;
    let mut signal = 0;

    let mut tick = |reg: i64| {
        if cycle == 20 || (cycle >= 60 && ((cycle - 60) % 40) == 0) {
            let s = cycle * reg;
            signal += s;
        }
        cycle += 1;
    };

    for line in lines.iter() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "noop" => tick(reg),
            "addx" => {
                tick(reg);
                tick(reg);
                reg += parts[1].parse::<i64>().unwrap();
            }
            _ => panic!("unknown instruction: {}", parts[0]),
        }
    }

    signal
}

fn _b(lines: Vec<String>, empty: char) -> String {
    let mut reg = 1i64;
    let mut cycle = 1i64;
    let width = 40usize;
    let mut display = vec![empty; width * 6];

    let mut tick = |reg: i64, display: &mut Vec<char>| {
        let row = (cycle - 1) / 40;
        let col = (cycle - 1) % 40;
        for i in -1..=1 {
            if col == (reg + i) {
                display[row as usize * width + col as usize] = '#';
            }
        }
        cycle += 1;
    };

    for line in lines.iter() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "noop" => tick(reg, &mut display),
            "addx" => {
                tick(reg, &mut display);
                tick(reg, &mut display);
                reg += parts[1].parse::<i64>().unwrap();
            }
            _ => panic!("unknown instruction: {}", parts[0]),
        }
    }

    for i in (0..display.len()).step_by(width+1) {
        display.insert(i, '\n');
    }

    display.iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::aoc::{load_lines, load_lines_suffix};

    use super::*;

    #[test]
    fn a() {
        assert_eq!(13140, _a(load_lines_suffix(10, "_test")))
    }

    #[test]
    fn a2() {
        assert_eq!(15680, _a(load_lines(10)))
    }

    #[test]
    fn b() {
        let expect = "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(expect, _b(load_lines_suffix(10, "_test"), '.'));
    }

    #[test]
    fn b2() {
        let expect = "
####.####.###..####.#..#..##..#..#.###..
...#.#....#..#.#....#..#.#..#.#..#.#..#.
..#..###..###..###..####.#....#..#.#..#.
.#...#....#..#.#....#..#.#.##.#..#.###..
#....#....#..#.#....#..#.#..#.#..#.#....
####.#....###..#....#..#..###..##..#....";
        assert_eq!(expect, _b(load_lines(10), '.'));
        // ZFBFHGUP
    }
}
