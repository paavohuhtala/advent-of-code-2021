use std::collections::BinaryHeap;

use itertools::Itertools;

use crate::array2d::Array2D;

const INPUT: &str = include_str!("./day15.txt");

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    cost: usize,
    position: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(map: &Array2D<u8>) -> Option<usize> {
    let start = 0usize;

    let goal = map.data.len() - 1;

    let mut dist = vec![usize::MAX; map.data.len()];
    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(Node {
        cost: 0,
        position: start,
    });

    while let Some(Node { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        if cost > dist[position] {
            continue;
        }

        let mut adjacent = Vec::new();
        map.adjacent_cardinal(position, &mut adjacent);

        for (adjacent_pos, adjacent_cost) in adjacent {
            let next = Node {
                cost: cost + adjacent_cost as usize,
                position: adjacent_pos,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    None
}

// https://doc.rust-lang.org/std/collections/binary_heap/index.html#examples
pub fn a() {
    let map = Array2D::from_string(INPUT);
    let lowest_cost = dijkstra(&map).unwrap();
    println!("Day15a: {}", lowest_cost);
}

fn add_risk(base: u8, add: u8) -> u8 {
    (((base - 1) + add) % 9) + 1
}

#[test]
fn test_risk() {
    assert_eq!(add_risk(1, 0), 1);
    assert_eq!(add_risk(1, 1), 2);
    assert_eq!(add_risk(1, 2), 3);
    assert_eq!(add_risk(9, 0), 9);
    assert_eq!(add_risk(9, 1), 1);
    assert_eq!(add_risk(9, 2), 2);
}

pub fn b() {
    let lines = INPUT
        .lines()
        .map(|line| {
            let mut modified_line: Vec<u8> = Vec::new();
            let elements = line
                .chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .collect_vec();

            for m in 0..=4 {
                modified_line.extend(elements.iter().copied().map(|e| add_risk(e, m)));
            }

            modified_line
        })
        .collect_vec();

    let mut map_lines = Vec::new();

    for m in 0..=4 {
        map_lines.extend(lines.iter().map(move |line| {
            line.iter()
                .copied()
                .map(move |e| add_risk(e, m))
                .collect_vec()
        }));
    }

    let map = Array2D::from_vec_vec(map_lines);

    let lowest_cost = dijkstra(&map).unwrap();
    println!("Day15b: {}", lowest_cost);
}
