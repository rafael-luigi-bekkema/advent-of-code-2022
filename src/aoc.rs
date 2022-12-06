use std::{fs::File, io::{BufReader, BufRead, Read}};

pub fn load_lines(day: usize) -> Vec<String> {
    let name = format!("inputs/day{:02}.txt", day);
    let f = File::open(&name).unwrap_or_else(|_| panic!("could not open file: {}", &name));
    BufReader::new(f)
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

pub fn load_text(day: usize) -> String {
    let name = format!("inputs/day{:02}.txt", day);
    let mut f = File::open(&name).unwrap_or_else(|_| panic!("could not open file: {}", &name));
    let mut out = String::new();
    f.read_to_string(&mut out).unwrap();
    out
}
