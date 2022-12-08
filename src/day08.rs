use crate::aoc::load_text;

pub fn a() -> u64 {
    _a(load_text(8))
}

pub fn b() -> u64 {
    _b(load_text(8))
}

fn _a(input: impl AsRef<str>) -> u64 {
    let input = input.as_ref();
    let width = input.find('\n').unwrap();
    let height = input.lines().count();
    let grid: Vec<u8> = input.replace('\n', "").bytes().collect();
    let mut visibles = Vec::new();

    for (i, val) in grid.iter().enumerate() {
        let x = i % width;
        let y = i / width;
        if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
            visibles.push(1);
            continue;
        }

        let mut blocked = [false; 4];
        for sx in 0..x {
            if grid[y * width + sx] >= *val {
                blocked[0] = true;
                break;
            }
        }
        for sx in x + 1..width {
            if grid[y * width + sx] >= *val {
                blocked[1] = true;
                break;
            }
        }
        for sy in 0..y {
            if grid[sy * width + x] >= *val {
                blocked[2] = true;
                break;
            }
        }
        for sy in y + 1..height {
            if grid[sy * width + x] >= *val {
                blocked[3] = true;
                break;
            }
        }

        visibles.push(if blocked.iter().all(|f| *f) { 0 } else { 1 });
    }

    visibles.iter().sum()
}

fn _b(input: impl AsRef<str>) -> u64 {
    let input = input.as_ref();
    let width = input.find('\n').unwrap();
    let height = input.lines().count();
    let grid: Vec<u8> = input.replace('\n', "").bytes().collect();
    let mut total_scores = Vec::new();

    for (i, val) in grid.iter().enumerate() {
        let x = i % width;
        let y = i / width;

        let mut scores = [0u64; 4];
        for sx in (0..x).rev() {
            scores[0] += 1;
            if grid[y * width + sx] >= *val {
                break;
            }
        }
        for sx in x + 1..width {
            scores[1] += 1;
            if grid[y * width + sx] >= *val {
                break;
            }
        }
        for sy in (0..y).rev() {
            scores[2] += 1;
            if grid[sy * width + x] >= *val {
                break;
            }
        }
        for sy in y + 1..height {
            scores[3] += 1;
            if grid[sy * width + x] >= *val {
                break;
            }
        }

        total_scores.push(scores[0] * scores[1] * scores[2] * scores[3]);
    }

    *total_scores.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::aoc::load_text;

    use super::*;

    #[test]
    fn a() {
        let input = "30373
25512
65332
33549
35390";
        assert_eq!(21, _a(input));
    }

    #[test]
    fn a2() {
        assert_eq!(1546, _a(load_text(8)));
    }

    #[test]
    fn b() {
        let input = "30373
25512
65332
33549
35390";
        assert_eq!(8, _b(input));
    }

    #[test]
    fn b2() {
        assert_eq!(519064, _b(load_text(8)));
    }
}
