use std::collections::{hash_map::Entry, HashMap};

use itertools::{Itertools, MinMaxResult};

const INPUT: &str = include_str!("./day14.txt");

struct Rule((char, char), char);

struct Input {
    template: Vec<char>,
    rules: Vec<Rule>,
}

fn read_input() -> Input {
    let (template, rules) = INPUT.split_once("\n\n").unwrap();
    let template = template.chars().collect_vec();

    let rules = rules
        .lines()
        .map(|line| {
            let (ab, c) = line.split_once(" -> ").unwrap();
            let [a, b]: [char; 2] = ab.chars().collect_vec().try_into().unwrap();
            Rule((a, b), c.chars().next().unwrap())
        })
        .collect_vec();
    Input { template, rules }
}

pub fn a() {
    let Input { rules, template } = read_input();

    let mut result = template;

    let mut insertions: Vec<(usize, char)> = Vec::new();

    for _ in 0..10 {
        for ((_, a), (i, b)) in result.iter().copied().enumerate().tuple_windows() {
            for rule in &rules {
                if rule.0 == (a, b) {
                    insertions.push((i, rule.1));
                }
            }
        }

        for (i, v) in insertions.drain(..).rev() {
            result.insert(i, v);
        }
    }

    let counts = result.iter().copied().counts().values().copied().minmax();
    match counts {
        MinMaxResult::MinMax(min, max) => {
            println!("Day14a {}", max - min);
        }
        _ => unreachable!(),
    }
}

pub fn b() {
    let Input { rules, template } = read_input();

    let mut element_counts = template.iter().copied().counts();
    let mut pair_counts: HashMap<(char, char), usize> =
        template.iter().copied().tuple_windows().counts();

    for _ in 0..2 {
        let mut next_element_counts = element_counts.clone();
        let mut next_counts = pair_counts.clone();

        for rule in &rules {
            match pair_counts.get(&rule.0) {
                None => continue,
                Some(&count) => {
                    next_counts.remove(&rule.0);
                    *next_element_counts.entry(rule.1).or_insert(0) += count;

                    let new_a = (rule.0 .0, rule.1);
                    let new_b = (rule.1, rule.0 .1);
                    *next_counts.entry(new_a).or_insert(0) += count;
                    *next_counts.entry(new_b).or_insert(0) += count;
                }
            }
        }

        element_counts = next_element_counts;
        pair_counts = next_counts;
    }

    println!("{:?}", element_counts);

    let counts = element_counts.values().copied().minmax();
    match counts {
        MinMaxResult::MinMax(min, max) => {
            println!("Day14b {}", max - min);
        }
        _ => unreachable!(),
    }
}
