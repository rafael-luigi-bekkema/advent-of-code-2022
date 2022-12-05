use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn load_file() -> Vec<String> {
    let f = File::open("inputs/day05.txt").unwrap();

    BufReader::new(f).lines().map(|l| l.unwrap()).collect()
}

pub fn a() -> String {
    _a(load_file())
}

pub fn b() -> String {
    _b(load_file())
}

fn parse_stacks(input: &Vec<String>) -> (Vec<Vec<char>>, usize) {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut line_nr = 0;
    for (l, line) in input.iter().enumerate() {
        if line == "" {
            line_nr = l;
            break;
        }
        for (i, char) in line.chars().enumerate() {
            if i == 0 || char == ' ' {
                continue;
            }
            if (i - 1) % 4 == 0 {
                let stack = (i - 1) / 4;
                while stack >= stacks.len() {
                    stacks.push(Vec::new());
                }
                stacks[stack].push(char);
            }
        }
    }

    for stack in stacks.iter_mut() {
        stack.pop();
        stack.reverse();
    }

    (stacks, line_nr)
}

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_move(line: &str) -> Move {
    let parts: Vec<&str> = line.split_whitespace().collect();
    Move {
        count: parts[1].parse::<usize>().unwrap(),
        from: parts[3].parse::<usize>().unwrap(),
        to: parts[5].parse::<usize>().unwrap(),
    }
}

fn _a(input: Vec<String>) -> String {
    let (mut stacks, line_nr) = parse_stacks(&input);

    for line in input[line_nr + 1..].iter() {
        let mov = parse_move(line);

        for _i in 0..mov.count {
            let item = stacks[mov.from - 1].pop().unwrap();
            stacks[mov.to - 1].push(item);
        }
    }

    let mut result = Vec::new();
    for stack in stacks.iter_mut() {
        let top = stack.pop().unwrap();
        result.push(top);
    }

    result.iter().collect()
}

fn _b(input: Vec<String>) -> String {
    let (mut stacks, line_nr) = parse_stacks(&input);

    for line in input[line_nr + 1..].iter() {
        let mov = parse_move(line);
        let len = stacks[mov.from - 1].len();

        let moved: Vec<char> = stacks[mov.from - 1]
            .splice(len - mov.count.., vec![])
            .collect();
        for item in moved {
            stacks[mov.to - 1].push(item);
        }
    }

    let mut result = Vec::new();
    for stack in stacks.iter_mut() {
        let top = stack.pop().unwrap();
        result.push(top);
    }

    result.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            .split_terminator("\n")
            .map(|l| l.to_string())
            .collect();
        assert_eq!("CMZ", _a(input))
    }

    #[test]
    fn a2() {
        assert_eq!("PSNRGBTFT", _a(load_file()));
    }

    #[test]
    fn b() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            .split_terminator("\n")
            .map(|l| l.to_string())
            .collect();
        assert_eq!("MCD", _b(input))
    }

    #[test]
    fn b2() {
        assert_eq!("BNTZFPMMW", _b(load_file()));
    }
}
