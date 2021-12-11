use std::collections::HashSet;

use itertools::Itertools;

use crate::array2d::Array2D;

const INPUT: &str = include_str!("./day9.txt");

fn read_input() -> Array2D<u8> {
    Array2D::from_string(INPUT)
}

fn get_adjacent(array: &Array2D<u8>, i: usize, output: &mut Vec<(usize, u8)>) {
    let (x, y) = array.i_to_coords(i);

    let left = array.try_get_left(x, y);
    let right = array.try_get_right(x, y);
    let up = array.try_get_up(x, y);
    let down = array.try_get_down(x, y);

    if let Some(left) = left {
        output.push(left);
    }
    if let Some(right) = right {
        output.push(right);
    }
    if let Some(up) = up {
        output.push(up);
    }
    if let Some(down) = down {
        output.push(down);
    }
}

fn find_low_points(array: &Array2D<u8>) -> Vec<(usize, u8)> {
    let mut low_points = Vec::new();

    let mut adjacent = Vec::new();
    for (i, x) in array.iter().enumerate() {
        get_adjacent(&array, i, &mut adjacent);

        if adjacent.drain(..).all(|(_, adj)| adj > x) {
            low_points.push((i, x));
        }
    }

    low_points
}

pub fn a() {
    let array = read_input();
    let risk_level = find_low_points(&array)
        .into_iter()
        .map(|(_, x)| (x + 1) as usize)
        .sum::<usize>();

    println!("Day9a: {}", risk_level);
}

pub fn b() {
    let array = read_input();
    let low_points = find_low_points(&array);

    let mut basin_sizes = Vec::new();

    for (i, _) in low_points {
        let mut visited = HashSet::<usize>::new();
        let mut queue = vec![i];

        while let Some(n) = queue.pop() {
            if !visited.contains(&n) {
                visited.insert(n);
                let mut adjacent = Vec::new();
                get_adjacent(&array, n, &mut adjacent);
                adjacent.retain(|(_, x)| *x < 9);
                queue.extend(adjacent.into_iter().map(|(i, _)| i));
            }
        }

        basin_sizes.push(visited.len());
    }

    let basin_sizes = basin_sizes.into_iter().sorted().rev().collect_vec();

    let answer: usize = basin_sizes.into_iter().take(3).product();
    println!("Day9b: {}", answer);
}
