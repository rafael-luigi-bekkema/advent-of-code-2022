use std::{fs::File, io::Write};

use crate::aoc::load_lines;

const EMPTY: u8 = b' ';
const WALL: u8 = b'#';
const SAND: u8 = b'o';
const SOURCE: u8 = b'+';

pub fn a() -> usize {
    _a(load_lines(14), "")
}

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

fn create_map(lines: Vec<&str>, with_floor: bool, extra_width: usize) -> (Vec<Vec<u8>>, Point) {
    let mut items = Vec::new();
    let (mut minx, mut maxx, mut miny, mut maxy) = (isize::MAX, 0isize, 0isize, 0isize);
    for line in lines.iter() {
        let mut subitems = Vec::new();
        for coords in line.split(" -> ") {
            let (x, y): (isize, isize) = coords
                .split_once(",")
                .map(|(s1, s2)| (s1.parse().unwrap(), s2.parse().unwrap()))
                .unwrap();
            if x < minx {
                minx = x;
            }
            if y < miny {
                miny = y;
            }
            if x > maxx {
                maxx = x;
            }
            if y > maxy {
                maxy = y;
            }
            subitems.push((x, y));
        }
        items.push(subitems);
    }

    println!("{} {} {} {}", minx, maxx, miny, maxy);

    if with_floor {
        maxy += 2;
    }

    let width = maxx - minx + 1 + (extra_width as isize);
    let height = maxy - miny + 1;
    minx -= (extra_width as isize) / 2;

    let mut grid = vec![vec![EMPTY; width as usize]; height as usize];

    for subitems in items.iter() {
        for (i, (fromx, fromy)) in subitems.iter().take(subitems.len() - 1).enumerate() {
            let (tox, toy) = subitems[i + 1];

            let rangey = if *fromy <= toy {
                *fromy..=toy
            } else {
                toy..=*fromy
            };
            let rangex = if *fromx <= tox {
                *fromx..=tox
            } else {
                tox..=*fromx
            };

            for y in rangey {
                for x in rangex.clone() {
                    grid[(y - miny) as usize][(x - minx) as usize] = WALL;
                }
            }
        }
    }

    if with_floor {
        let y = height-1;
        for x in 0..width {
            grid[y as usize][x as usize] = WALL;
        }
    }

    let source = Point{x: (500 - minx) as usize, y: 0};
    grid[source.y][source.x] = SOURCE;

    (grid, source)
}

fn _b(lines: Vec<impl AsRef<str>>, suffix: &str, extra_width: usize) -> usize {
    let (mut grid, start) = create_map(lines.iter().map(|l| l.as_ref()).collect(), true, extra_width);
    let mut count = 0;

    'outer: loop {
        let mut pos = start;
        loop {
            while grid[pos.y+1][pos.x] == EMPTY {
                pos.y += 1;
            };
            if grid[pos.y+1][pos.x-1] == EMPTY {
                pos.x -= 1;
                pos.y += 1;
                continue;
            }
            if grid[pos.y+1][pos.x+1] == EMPTY {
                pos.x += 1;
                pos.y += 1;
                continue;
            }
            break;
        }

        grid[pos.y][pos.x] = SAND;
        count += 1;

        if pos == start {
            break;
        }
    }

    print_map(&grid, suffix);

    count
}


fn _a(lines: Vec<impl AsRef<str>>, suffix: &str) -> usize {
    let (mut grid, start) = create_map(lines.iter().map(|l| l.as_ref()).collect(), false, 2);
    let mut count = 0;

    'outer: loop {
        let mut pos = start;
        loop {
            if pos.y+1 == grid.len() {
                break 'outer
            }

            while grid[pos.y+1][pos.x] == EMPTY {
                pos.y += 1;
            };
            if grid[pos.y+1][pos.x-1] == EMPTY {
                pos.x -= 1;
                pos.y += 1;
                continue;
            }
            if grid[pos.y+1][pos.x+1] == EMPTY {
                pos.x += 1;
                pos.y += 1;
                continue;
            }
            break;
        }

        grid[pos.y][pos.x] = SAND;
        count += 1;
    }

    print_map(&grid, suffix);

    count
}

fn print_map(grid: &Vec<Vec<u8>>, suffix: &str) {
    let mut file = File::create(format!("output/day14{suffix}.txt")).unwrap();

    for (_, line) in grid.iter().enumerate() {
        file.write(line).unwrap();
        file.write(&[b'\n']).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::load_lines;

    use super::*;

    #[test]
    fn a() {
        assert_eq!(
            24,
            _a(
                vec![
                    "498,4 -> 498,6 -> 496,6",
                    "503,4 -> 502,4 -> 502,9 -> 494,9",
                ],
                "a_test"
            )
        );
    }

    #[test]
    fn a2() {
        assert_eq!(
            614,
            _a(load_lines(14), "a")
        );
    }

    #[test]
    fn b() {
        assert_eq!(
            93,
            _b(
                vec![
                    "498,4 -> 498,6 -> 496,6",
                    "503,4 -> 502,4 -> 502,9 -> 494,9",
                ],
                "b_test",
                22
            )
        );
    }

    #[test]
    fn b2() {
        assert_eq!(
            26170,
            _b(
                load_lines(14),
                "b",
                302
            )
        );
    }
}
