use std::collections::{BTreeMap, HashMap};

use crate::aoc::load_lines;

pub fn a() -> usize {
    _a(load_lines(12))
}

type Map = Vec<Vec<char>>;

fn parse_map(input: Vec<&str>) -> Map {
    let mut result = Vec::new();
    for line in input {
        result.push(line.chars().collect());
    }
    result
}

fn elevation(value: char) -> char {
    match value {
        'S' => 'a',
        'E' => 'z',
        _ => value,
    }
}

fn is_edge(from: char, to: char) -> bool {
    (elevation(to) as isize - elevation(from) as isize) <= 1
}

fn node_id(width: usize, y: usize, x: usize) -> usize {
    y * width + x
}

fn create_nodes(map: Map) -> (BTreeMap<usize, Node>, usize, usize) {
    let mut out = BTreeMap::new();
    let mut target = 0usize;
    let mut pos = 0usize;
    let width = map[0].len();
    let height = map.len();

    for (y, row) in map.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            let num = node_id(width, y, x);
            match *val {
                'S' => pos = num,
                'E' => target = num,
                _ => {}
            }

            let mut edges = Vec::new();

            if y > 0 && is_edge(map[y][x], map[y - 1][x]) {
                edges.push(node_id(width, y - 1, x));
            }
            if (y + 1) < height && is_edge(map[y][x], map[y + 1][x]) {
                edges.push(node_id(width, y + 1, x));
            }
            if x > 0 && is_edge(map[y][x], map[y][x - 1]) {
                edges.push(node_id(width, y, x - 1));
            }
            if (x + 1) < width && is_edge(map[y][x], map[y][x + 1]) {
                edges.push(node_id(width, y, x + 1));
            }

            out.insert(num, Node { value: num, edges });
        }
    }

    (out, pos, target)
}

struct Node {
    value: usize,
    edges: Vec<usize>,
}

fn dijkstra(
    nodes: BTreeMap<usize, Node>,
    source: usize,
) -> (HashMap<usize, usize>, HashMap<usize, Option<usize>>) {
    let mut distance = HashMap::new();
    let mut prev = HashMap::new();
    let mut q = Vec::new();
    let mut visited = HashMap::new();

    for (_, node) in nodes.iter() {
        distance.insert(node.value, usize::MAX);
        prev.insert(node.value, None);

        q.push(node.value);
    }

    distance.insert(source, 0);

    while !q.is_empty() {
        q.sort_by(|a, b| distance[b].cmp(&distance[a]));

        let u = &nodes[&q.pop().unwrap()];

        visited.insert(u.value, true);

        for vc in u.edges.iter() {
            if visited.contains_key(vc) {
                continue;
            }
            if !nodes.contains_key(vc) {
                println!("key: {}", *vc);
            }
            let v = &nodes[vc];
            let temp_dist = distance[&u.value].saturating_add(1);
            if temp_dist < distance[&v.value] {
                distance.insert(v.value, temp_dist);
                prev.insert(v.value, Some(u.value));
            }
        }
    }

    (distance, prev)
}

fn _a(input: Vec<impl AsRef<str>>) -> usize {
    let map = parse_map(input.iter().map(|s| s.as_ref()).collect());
    let (map, start, target) = create_nodes(map);
    let (dist, _prev) = dijkstra(map, start);

    dist[&target]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(31, _a(input.split('\n').collect()));
    }

    #[test]
    fn a2() {
        assert_eq!(370, _a(load_lines(12)));
    }
}
