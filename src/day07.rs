use std::collections::HashMap;

use crate::aoc::load_lines;

pub fn a() -> u64 {
    _a(load_lines(7))
}

pub fn b() -> u64 {
    _b(load_lines(7))
}

fn path(cwd: &[&str]) -> String {
    let dir = cwd.join("/");
    match dir.as_ref() {
        "/" => dir,
        _ => dir.chars().skip(1).collect(), // Remove leading / or we get double /
    }
}

fn parse_sizes(lines: Vec<impl AsRef<str>>) -> HashMap<String, u64> {
    let mut cwd = Vec::new();
    let mut sizes = HashMap::new();

    for line in lines.iter() {
        let line: Vec<&str> = line.as_ref().split_whitespace().collect();
        if line[0] == "$" {
            if line[1] == "cd" {
                if line[2] == ".." {
                    cwd.pop().unwrap();
                } else {
                    cwd.push(line[2])
                }
            }
        } else if line[0] != "dir" {
            let size = line[0].parse::<u64>().unwrap();
            let entry = sizes.entry(path(&cwd)).or_insert(0);
            *entry += size;

            for i in 1..cwd.len() {
                let subpath: Vec<&str> = cwd.iter().take(cwd.len() - i).copied().collect();
                let entry = sizes.entry(path(&subpath)).or_insert(0);
                *entry += size;
            }
        }
    }

    sizes
}

fn _a(lines: Vec<impl AsRef<str>>) -> u64 {
    let sizes = parse_sizes(lines);

    sizes.values().filter(|v| **v <= 100_000).sum()
}

fn _b(lines: Vec<impl AsRef<str>>) -> u64 {
    let sizes = parse_sizes(lines);

    let total_space = 70_000_000;
    let need_space = 30_000_000;

    let free_up = need_space - (total_space - sizes["/"]);

    let mut min_dir = String::new();
    let mut min_diff: u64 = 0;
    let mut first = true;

    for (dir, size) in sizes.iter() {
        if *size < free_up {
            continue;
        }
        let diff = *size - free_up;
        if first || diff < min_diff {
            first = false;
            min_diff = diff;
            min_dir = dir.clone();
        }
    }

    sizes[&min_dir]
}

#[cfg(test)]
mod tests {
    use crate::aoc::load_lines;

    use super::*;

    #[test]
    fn a() {
        let input: Vec<&str> = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
            .split("\n")
            .collect();
        assert_eq!(95437, _a(input));
    }

    #[test]
    fn a2() {
        assert_eq!(1501149, _a(load_lines(7)));
    }

    #[test]
    fn b() {
        let input: Vec<&str> = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
            .split("\n")
            .collect();
        assert_eq!(24933642, _b(input));
    }

    #[test]
    fn b2() {
        assert_eq!(10096985, _b(load_lines(7)));
    }
}
