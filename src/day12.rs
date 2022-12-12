use std::{collections::{BTreeMap, HashMap, BinaryHeap, HashSet}, cmp::Ordering};

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

            out.insert(num, Node { id: num, edges });
        }
    }

    (out, pos, target)
}

struct Node {
    id: usize,
    edges: Vec<usize>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    distance: usize,
    id: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

fn dijkstra(
    nodes: &BTreeMap<usize, Node>,
    source: usize,
) -> (HashMap<usize, usize>, HashMap<usize, Option<usize>>) {
    let mut distance = HashMap::new();
    let mut prev = HashMap::new();
    let mut q = BinaryHeap::new();
    let mut visited = HashSet::new();

    for (_, node) in nodes.iter() {
        distance.insert(node.id, usize::MAX);
        prev.insert(node.id, None);
    }

    distance.insert(source, 0);
    q.push(State{distance: 0, id: source});

    while let Some(u) = q.pop() {
        visited.insert(u.id);

        for vc in nodes[&u.id].edges.iter() {
            if visited.contains(vc) {
                continue;
            }
            let v = &nodes[vc];
            let temp_dist = distance[&u.id].saturating_add(1);
            if temp_dist < distance[&v.id] {
                q.push(State{distance: temp_dist, id: v.id});
                distance.insert(v.id, temp_dist);
                prev.insert(v.id, Some(u.id));
            }
        }
    }

    (distance, prev)
}

fn _a(input: Vec<impl AsRef<str>>) -> usize {
    let map = parse_map(input.iter().map(|s| s.as_ref()).collect());
    let (map, start, target) = create_nodes(map);
    let (dist, _prev) = dijkstra(&map, start);

    dist[&target]
}

fn _b(input: Vec<impl AsRef<str>>) -> usize {
    let map = parse_map(input.iter().map(|s| s.as_ref()).collect());

    let mut starts = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val == 'S' || *val == 'a' {
                starts.push(y * row.len() +  x);
            }
        }
    }

    let (map, _, target) = create_nodes(map);

    let mut min_dist = usize::MAX;
    for start in starts {
        let (dist, _prev) = dijkstra(&map, start);
        if dist[&target] < min_dist {
            min_dist = dist[&target]
        }
    }

    min_dist
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

    #[test]
    fn b() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(29, _b(input.split('\n').collect()));
    }

    #[test]
    fn b2() {
        assert_eq!(363, _b(load_lines(12)));
    }
}
