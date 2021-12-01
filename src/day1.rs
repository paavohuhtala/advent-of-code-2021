use crate::util::parse_lines;
use itertools::Itertools;

const INPUT: &str = include_str!("./day1.txt");

fn day1a() {
    let lines = parse_lines(INPUT);

    let total_increases: i32 = lines
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| if b > a { 1 } else { 0 })
        .sum();

    println!("Day 1a: {}", total_increases);
}

fn day1b() {
    let lines = parse_lines(INPUT);

    let total_increases: i32 = lines
        .into_iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .map(|(a, b)| if b > a { 1 } else { 0 })
        .sum();

    println!("Day 1b: {}", total_increases);
}

pub fn day1() {
    day1a();
    day1b();
}
