use std::collections::HashMap;

const INPUT: &str = include_str!("./day5.txt");

type Point = (i32, i32);
type Line = (Point, Point);

fn parse_input() -> Vec<Line> {
    INPUT
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(" -> ").unwrap();
            let (ax, ay) = a.split_once(",").unwrap();
            let (bx, by) = b.split_once(",").unwrap();
            (
                (ax.parse().unwrap(), ay.parse().unwrap()),
                (bx.parse().unwrap(), by.parse().unwrap()),
            )
        })
        .collect()
}

fn mark_covered_points(counts: &mut HashMap<Point, u32>, line: &Line, include_diagonal: bool) {
    if line.0 .0 == line.1 .0 {
        let start = line.0 .1.min(line.1 .1);
        let end = line.0 .1.max(line.1 .1);

        for y in start..=end {
            let coord = (line.0 .0, y);
            *counts.entry(coord).or_insert(0) += 1;
        }
    } else if line.0 .1 == line.1 .1 {
        let start = line.0 .0.min(line.1 .0);
        let end = line.0 .0.max(line.1 .0);

        for x in start..=end {
            let coord = (x, line.0 .1);
            *counts.entry(coord).or_insert(0) += 1;
        }
    } else if include_diagonal {
        let start = line.0 .0.min(line.1 .0);
        let end = line.0 .0.max(line.1 .0);

        for x in start..=end {
            let y = (line.0 .1 as f32
                + (x as f32 - line.0 .0 as f32) * (line.1 .1 as f32 - line.0 .1 as f32)
                    / (line.1 .0 as f32 - line.0 .0 as f32))
                .round() as i32;
            let coord = (x, y);
            *counts.entry(coord).or_insert(0) += 1;
        }
    }
}

// Prints a 2D grid representation of the board
// Unmarked cells are marked with a '.'
// Marked cells use the correspoding number
fn print_board(counts: &HashMap<Point, u32>) {
    let mut min_x = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut min_y = std::i32::MAX;
    let mut max_y = std::i32::MIN;

    for (coord, _) in counts.iter() {
        min_x = coord.0.min(min_x);
        max_x = coord.0.max(max_x);
        min_y = coord.1.min(min_y);
        max_y = coord.1.max(max_y);
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let coord = (x, y);
            let count = counts.get(&coord).unwrap_or(&0);

            if *count == 0 {
                print!(".");
            } else {
                print!("{}", count);
            }
        }
        println!();
    }
}

fn count_overlapping_points(include_diagonal: bool) -> usize {
    let lines = parse_input();
    let mut counts = HashMap::new();

    for line in lines.iter() {
        mark_covered_points(&mut counts, line, include_diagonal);
    }

    counts.into_values().filter(|&value| value >= 2).count()
}

pub fn a() {
    println!("Day5a: {}", count_overlapping_points(false));
}

pub fn b() {
    println!("Day5b: {}", count_overlapping_points(true));
}
