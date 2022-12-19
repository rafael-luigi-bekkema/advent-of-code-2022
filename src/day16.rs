use std::collections::HashMap;

use crate::aoc::{load_lines, load_lines_suffix};
use regex::Regex;

pub fn a() -> usize {
    _a(load_lines_suffix(16, ""))
}

#[derive(Debug)]
struct Valve {
    name: String,
    rate: usize,
    lead_to: Vec<String>,
}

fn parse_valves(lines: &Vec<String>) -> HashMap<String, Valve> {
    let re =
        Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();

    lines
        .iter()
        .map(|s| {
            let captures = re.captures(s).unwrap();
            let name = captures.get(1).unwrap().as_str().to_string();
            (
                name.clone(),
                Valve {
                    name,
                    rate: captures.get(2).unwrap().as_str().parse().unwrap(),
                    lead_to: captures
                        .get(3)
                        .unwrap()
                        .as_str()
                        .split(", ")
                        .map(|s| s.to_string())
                        .collect(),
                },
            )
        })
        .collect()
}

#[derive(Hash, Eq, PartialEq)]
struct HashKey {
    open: Vec<String>,
    pos: String,
}

fn follow(
    valves: &HashMap<String, Valve>,
    valve: &str,
    minute: usize,
    flow: usize,
    released: usize,
    mut open: Vec<String>,
    mut visited: HashMap<String, usize>,
    max: &mut usize,
    full_flow: usize,
) {
    if minute > 30 {
        if released > *max {
            *max = released;
            println!(
                "max: {}, minute: {}, flow: {}, open: {:?}",
                *max, minute, flow, open
            );
        }
        return;
    }

    if let Some(val) = visited.get(valve) {
        if *val == flow {
            return;
        }
    }
    visited.insert(valve.to_string(), flow);

    if *max > 0 && released + full_flow * (30 - minute + 1) < *max {
        return
    }

    let v = &valves[valve];

    let extra_time = 1;
    let extra_flow = 0;

    // First continue without opening valve
    for svalve in v.lead_to.iter() {
        follow(
            valves,
            svalve,
            minute + extra_time,
            flow + extra_flow,
            released + (extra_time * flow),
            open.clone(),
            visited.clone(),
            max,
            full_flow,
        );
    }

    if v.rate == 0 || minute == 30 || open.contains(&v.name) {
        return;
    }

    // Continue with opening of valve
    let extra_time = 2;
    let extra_flow = v.rate;
    open.push(v.name.clone());

    for svalve in v.lead_to.iter() {
        follow(
            valves,
            svalve,
            minute + extra_time,
            flow + extra_flow,
            released + (extra_time * flow),
            open.clone(),
            visited.clone(),
            max,
            full_flow,
        );
    }
}

fn _a(lines: Vec<String>) -> usize {
    let valves = parse_valves(&lines);
    let mut max = 0;
    let full_flow = valves.iter().map(|(_, v)| v.rate).sum();
    follow(&valves, "AA", 0, 0, 0, Vec::new(), HashMap::new(), &mut max, full_flow);
    max
}

#[cfg(test)]
mod tests {
    use crate::aoc::load_lines_suffix;

    use super::*;

    #[test]
    fn a() {
        assert_eq!(1651, _a(load_lines_suffix(16, "_test")))
    }

    // #[test]
    // fn a2() {
    //     // max found: 2080
    //     assert_eq!(0, _a(load_lines(16)))
    // }
}
