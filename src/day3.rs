use itertools::Itertools;

const INPUT: &str = include_str!("./day3.txt");
const BITS: usize = 12;

fn read_input() -> Vec<([bool; BITS], &'static str)> {
    INPUT
        .lines()
        .map(|line| {
            (
                line.chars()
                    .map(|c| match c {
                        '0' => false,
                        '1' => true,
                        _ => panic!(),
                    })
                    .collect_vec()
                    .try_into()
                    .unwrap(),
                line,
            )
        })
        .collect_vec()
}

pub fn a() {
    let input = read_input();
    let mut bit_popularities = [(0, 0); BITS];

    for row in input {
        for (i, bit) in row.0.into_iter().enumerate() {
            if bit {
                bit_popularities[i].1 += 1;
            } else {
                bit_popularities[i].0 += 1;
            }
        }
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for (zero, one) in bit_popularities {
        if zero > one {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }

    let gamma = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon, 2).unwrap();

    println!("Day 3a: {}", gamma * epsilon);
}

fn find_rating(
    mut input: Vec<([bool; BITS], &'static str)>,
    criteria: fn(i32, i32) -> bool,
) -> u32 {
    let mut i = 0;

    while input.len() > 1 {
        let mut zeroes = 0;
        let mut ones = 0;

        for (bits, _) in &input {
          if bits[i] {
            ones += 1;
          } else {
            zeroes += 1;
          }
        }

        let winner = criteria(zeroes, ones);
        input.retain(|(bits, _)| bits[i] == winner);
        i += 1;
    }

    let rating = u32::from_str_radix(&input[0].1, 2).unwrap();
    rating
}

pub fn b() {
    let input = read_input();

    let o2 = find_rating(input.clone(), |zero, one| {
        if zero == one {
            true
        } else {
            one > zero
        }
    });
    let co2 = find_rating(input.clone(), |zero, one| {
        if zero == one {
            false
        } else {
            one < zero
        }
    });

    println!("O2: {}", o2);
    println!("CO2: {}", co2);

    println!("Day 3b: {}", o2 * co2);
}
