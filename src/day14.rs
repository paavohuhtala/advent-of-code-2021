use itertools::Itertools;

const INPUT: &str = include_str!("./day14.txt");

#[derive(Debug)]
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

fn solve(iterations: usize) -> usize {
    let Input { rules, template } = read_input();

    let mut element_counts = template.iter().copied().counts();
    let mut pair_counts = template.iter().copied().tuple_windows().counts();

    for _ in 0..iterations {
        let mut next_element_counts = element_counts.clone();
        let mut next_counts = pair_counts.clone();

        for rule in &rules {
            match pair_counts.get(&rule.0) {
                None => continue,
                Some(&count) => {
                    *next_counts.entry(rule.0).or_insert(0) -= count;
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

    let (min, max) = element_counts
        .values()
        .copied()
        .minmax()
        .into_option()
        .unwrap();

    max - min
}

pub fn ab() {
    println!("Day14a {}", solve(10));
    println!("Day14b {}", solve(40));
}
