use std::{
    cmp::Ordering,
    collections::BinaryHeap,
};

use crate::aoc::load_lines;

pub fn a() -> usize {
    _a(load_lines(12))
}

pub fn b() -> usize {
    _b(load_lines(12))
}

fn create_nodes(
    map: Vec<&[u8]>,
    start_markers: &[u8],
) -> (Vec<Node>, Vec<usize>, usize) {
    let mut out = Vec::new();
    let mut target = 0usize;
    let mut starts = Vec::new();
    let width = map[0].len();
    let height = map.len();

    let node_id = |y, x| y * width + x;

    let elevation = |value| match value {
        b'S' => b'a',
        b'E' => b'z',
        _ => value,
    };

    let is_edge = |fx: usize, fy: usize, tx: usize, ty: usize| {
        let from = elevation(map[fy][fx]) as isize;
        let to = elevation(map[ty][tx]) as isize;

        to - from <= 1
    };

    for (y, row) in map.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            let num = node_id(y, x);
            if *val == b'E' {
                target = num;
            }

            if start_markers.contains(val) {
                starts.push(num);
            }

            let mut edges = Vec::new();

            if y > 0 && is_edge(x, y, x, y - 1) {
                edges.push(node_id(y - 1, x));
            }
            if (y + 1) < height && is_edge(x, y, x, y + 1) {
                edges.push(node_id(y + 1, x));
            }
            if x > 0 && is_edge(x, y, x - 1, y) {
                edges.push(node_id(y, x - 1));
            }
            if (x + 1) < width && is_edge(x, y, x + 1, y) {
                edges.push(node_id(y, x + 1));
            }

            out.push(Node { id: num, edges });
        }
    }

    (out, starts, target)
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

fn dijkstra(nodes: &[Node], source: usize, target: usize) -> Vec<usize> {
    let mut q = BinaryHeap::new();

    let mut distance = vec![usize::MAX; nodes.len()];
    distance[source] = 0;

    q.push(State {
        distance: 0,
        id: source,
    });

    while let Some(u) = q.pop() {
        if u.id == target {
            break;
        }

        for vc in nodes[u.id].edges.iter() {
            if distance[*vc] < usize::MAX {
                continue;
            }
            let v = &nodes[*vc];
            let temp_dist = distance[u.id].saturating_add(1);
            if temp_dist < distance[v.id] {
                q.push(State {
                    distance: temp_dist,
                    id: v.id,
                });
                distance[v.id] = temp_dist;
            }
        }
    }

    distance
}

fn _a(input: Vec<impl AsRef<str>>) -> usize {
    let (map, starts, target) = create_nodes(
        input.iter().map(|s| s.as_ref().as_bytes()).collect(),
        &[b'S'],
    );
    let dist = dijkstra(&map, starts[0], target);

    dist[target]
}

fn _b(input: Vec<impl AsRef<str>>) -> usize {
    let (map, starts, target) = create_nodes(
        input.iter().map(|s| s.as_ref().as_bytes()).collect(),
        &[b'S', b'a'],
    );

    let mut min_dist = usize::MAX;
    for start in starts {
        let dist = dijkstra(&map, start, target);
        if dist[target] < min_dist {
            min_dist = dist[target]
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
