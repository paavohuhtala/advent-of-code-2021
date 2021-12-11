use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!("./day9.txt");

struct Array2D<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Array2D<T>
where
    T: Copy + 'static,
{
    fn from_data_and_rows(data: Vec<T>, rows: usize) -> Self {
        let width = data.len() / rows;
        Self {
            data,
            width,
            height: rows,
        }
    }

    fn get(&self, x: i32, y: i32) -> T {
        self.data[(y as usize) * self.width + (x as usize)]
    }

    fn try_get_with_i(&self, x: i32, y: i32) -> Option<(usize, T)> {
        if x >= 0 && y >= 0 && x < (self.width as i32) && y < (self.height as i32) {
            Some(((y as usize) * self.width + (x as usize), self.get(x, y)))
        } else {
            None
        }
    }

    fn try_get_left(&self, x: i32, y: i32) -> Option<(usize, T)> {
        self.try_get_with_i(x - 1, y)
    }

    fn try_get_right(&self, x: i32, y: i32) -> Option<(usize, T)> {
        self.try_get_with_i(x + 1, y)
    }

    fn try_get_up(&self, x: i32, y: i32) -> Option<(usize, T)> {
        self.try_get_with_i(x, y - 1)
    }

    fn try_get_down(&self, x: i32, y: i32) -> Option<(usize, T)> {
        self.try_get_with_i(x, y + 1)
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = T> + 'a {
        self.data.iter().map(|x| *x)
    }

    fn i_to_coords(&self, i: usize) -> (i32, i32) {
        ((i % self.width) as i32, (i / self.width) as i32)
    }
}

fn read_input() -> Array2D<u8> {
    let rows = INPUT.lines().count();

    let data = INPUT
        .lines()
        .flat_map(|line| line.chars().map(|char| char.to_digit(10).unwrap() as u8))
        .collect_vec();

    Array2D::from_data_and_rows(data, rows)
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
