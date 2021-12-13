use std::collections::HashSet;

use itertools::Itertools;
use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("./day13.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Axis {
    X,
    Y,
}

#[derive(Debug)]
struct Input {
    points: Vec<(i32, i32)>,
    folds: Vec<(Axis, i32)>,
}

type World = HashSet<(i32, i32)>;

fn read_input() -> Input {
    let (points, folds) = INPUT.split_once("\n\n").unwrap();

    let points = points
        .lines()
        .map(|line| {
            let (l, r) = scan_fmt!(line, "{d},{d}", i32, i32).unwrap();
            (l, r)
        })
        .collect_vec();

    let folds = folds
        .lines()
        .map(|line| {
            let (axis, value) = scan_fmt!(line, "fold along {[xy]}={d}", char, i32).unwrap();

            let axis = match axis {
                'x' => Axis::X,
                'y' => Axis::Y,
                _ => panic!("invalid axis"),
            };

            (axis, value)
        })
        .collect_vec();

    Input { points, folds }
}

fn fold_points(world: World, axis: Axis, value: i32) -> World {
    let mut new_world = HashSet::new();

    for point in world.into_iter() {
        let point = fold_point(axis, value, point);
        new_world.insert(point);
    }

    new_world
}

fn fold_point(axis: Axis, value: i32, (x, y): (i32, i32)) -> (i32, i32) {
    let point = match axis {
        Axis::X => {
            if x < value {
                (x, y)
            } else {
                let x = value - (x - value);
                (x, y)
            }
        }
        Axis::Y => {
            if y < value {
                (x, y)
            } else {
                let y = value - (y - value);
                (x, y)
            }
        }
    };
    point
}

pub fn print_world(world: &World) {
    let min_x = world.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = world.iter().map(|(x, _)| *x).max().unwrap();
    let min_y = world.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = world.iter().map(|(_, y)| *y).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if world.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

pub fn a() {
    let input = read_input();
    let world = input.points.into_iter().collect();

    let (axis, value) = input.folds[0];
    let new_world = fold_points(world, axis, value);
    println!("Day13a: {}", new_world.len());
}

pub fn b() {
    let input = read_input();
    let world = input.points.into_iter().collect();

    let world = input.folds.into_iter().fold(world, |world, (axis, value)| {
        let new_world = fold_points(world, axis, value);
        new_world
    });

    println!("Day13b:");
    print_world(&world);
}
