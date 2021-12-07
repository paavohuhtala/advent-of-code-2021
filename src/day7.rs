const INPUT: &str = include_str!("./day7.txt");

fn read_input() -> Vec<i32> {
    INPUT.split(',').map(|s| s.parse().unwrap()).collect()
}

pub fn a() {
    let input = read_input();

    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    let mut smallest_diff = i32::MAX;

    for i in min..=max {
        let mut diff = 0;

        for &crab in &input {
            diff += (i - crab).abs();
        }

        if diff < smallest_diff {
            smallest_diff = diff;
        }
    }

    println!("Day7a: {}", smallest_diff);
}

pub fn b() {
    let input = read_input();

    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    let mut smallest_diff = i32::MAX;

    for i in min..=max {
        let mut diff = 0;

        for &crab in &input {
            // sum of arithmetic series
            let n = (i - crab).abs();
            let sum = n * (1 + n) / 2;
            diff += sum;
        }

        if diff < smallest_diff {
            smallest_diff = diff;
        }
    }

    println!("Day7b: {}", smallest_diff);
}
