use core::panic;
use std::collections::{HashMap, HashSet};

use bitflags::bitflags;
use itertools::Itertools;
use retain_mut::RetainMut;

const INPUT: &str = include_str!("./day8.txt");

bitflags! {
  struct Segments: u8 {
    const NONE = 0b0000;
    const A = 0b0000_0001;
    const B = 0b0000_0010;
    const C = 0b0000_0100;
    const D = 0b0000_1000;
    const E = 0b0001_0000;
    const F = 0b0010_0000;
    const G = 0b0100_0000;

    const ZERO = Self::A.bits | Self::B.bits | Self::C.bits | Self::E.bits | Self::F.bits | Self::G.bits;
    const ONE = Self::C.bits | Self::F.bits;
    const TWO = Self::A.bits | Self::C.bits | Self::D.bits | Self::E.bits | Self::G.bits;
    const THREE = Self::A.bits | Self::C.bits | Self::D.bits | Self::F.bits | Self::G.bits;
    const FOUR = Self::B.bits | Self::C.bits | Self::D.bits | Self::F.bits;
    const FIVE = Self::A.bits | Self::B.bits | Self::D.bits | Self::F.bits | Self::G.bits;
    const SIX = Self::A.bits | Self::B.bits | Self::D.bits | Self::E.bits | Self::F.bits | Self::G.bits;
    const SEVEN = Self::A.bits | Self::C.bits | Self::F.bits;
    const EIGHT = Self::A.bits | Self::B.bits | Self::C.bits | Self::D.bits | Self::E.bits | Self::F.bits | Self::G.bits;
    const NINE = Self::A.bits | Self::B.bits | Self::C.bits | Self::D.bits | Self::F.bits | Self::G.bits;
  }
}

impl Segments {
    fn from_char(c: char) -> Segments {
        match c {
            'a' => Segments::A,
            'b' => Segments::B,
            'c' => Segments::C,
            'd' => Segments::D,
            'e' => Segments::E,
            'f' => Segments::F,
            'g' => Segments::G,
            _ => panic!("Invalid segment: {}", c),
        }
    }

    fn count_bits(self) -> usize {
        self.bits().count_ones() as usize
    }
}

fn group_to_segments(group: &str) -> Segments {
    group
        .chars()
        .map(Segments::from_char)
        .fold(Segments::NONE, |acc, segment| acc | segment)
}

struct Line(String, [Segments; 10], [Segments; 4]);

fn read_input() -> Vec<Line> {
    INPUT
        .lines()
        .map(|line| {
            let (l, r) = line.split_once('|').unwrap();
            let l = l.trim();
            let r = r.trim();

            let input_segment_groups: [Segments; 10] = l
                .split(' ')
                .map(group_to_segments)
                .collect_vec()
                .try_into()
                .unwrap();
            let output_segment_groups: [Segments; 4] = r
                .split(' ')
                .map(group_to_segments)
                .collect_vec()
                .try_into()
                .unwrap();

            Line(
                line.to_string(),
                input_segment_groups,
                output_segment_groups,
            )
        })
        .collect()
}

fn infer_obvious(line: &Line) -> ([Segments; 4], Vec<Segments>) {
    let mut one = Segments::NONE;
    let mut four = Segments::NONE;
    let mut seven = Segments::NONE;
    let mut eight = Segments::NONE;

    let mut items = line.1.to_vec();

    items.retain_mut(|&mut i| {
        let bits = i.count_bits();

        let inferred_digit = match bits {
            2 => Some(&mut one),
            4 => Some(&mut four),
            3 => Some(&mut seven),
            7 => Some(&mut eight),
            _ => None,
        };

        if let Some(inferred_digit) = inferred_digit {
            *inferred_digit = i;
            false
        } else {
            true
        }
    });

    assert_eq!(items.len(), 6);

    ([one, four, seven, eight], items)
}

fn infer_and_count_obvious_digits(line: &Line) -> usize {
    let (known_digits, _) = infer_obvious(line);
    line.2.iter().filter(|d| known_digits.contains(*d)).count()
}

fn solve_line(line: &Line) -> u32 {
    let ([one, four, seven, eight], mut rest) = infer_obvious(line);

    let mut three = Segments::NONE;
    let mut six = Segments::NONE;

    rest.retain(|&i| {
        let xor_one = (i ^ one).count_bits();

        match xor_one {
            3 => {
                three = i;
                false
            }
            6 => {
                six = i;
                false
            }
            _ => true,
        }
    });

    assert_ne!(three, Segments::NONE);
    assert_ne!(six, Segments::NONE);

    let mut nine = Segments::NONE;

    rest.retain(|&i| {
        let xor_three = (i ^ three).count_bits();

        match xor_three {
            1 => {
                nine = i;
                false
            }
            _ => true,
        }
    });

    assert_ne!(nine, Segments::NONE);

    let mut two = Segments::NONE;
    let mut zero = Segments::NONE;
    let mut five = Segments::NONE;

    rest.retain(|&i| {
        let xor_six = (i ^ six).count_bits();

        match xor_six {
            1 => {
                five = i;
                false
            }
            2 => {
                zero = i;
                false
            }
            3 => {
                two = i;
                false
            }
            _ => true,
        }
    });

    assert_ne!(two, Segments::NONE);
    assert_ne!(zero, Segments::NONE);
    assert_ne!(five, Segments::NONE);

    assert!(rest.is_empty());

    assert_eq!(
        [zero, one, two, three, four, five, six, seven, eight, nine]
            .into_iter()
            .collect::<HashSet<_>>()
            .len(),
        10
    );

    let decoded_digits: HashMap<Segments, char> = [
        (zero, '0'),
        (one, '1'),
        (two, '2'),
        (three, '3'),
        (four, '4'),
        (five, '5'),
        (six, '6'),
        (seven, '7'),
        (eight, '8'),
        (nine, '9'),
    ]
    .into_iter()
    .collect();

    let code = line
        .2
        .into_iter()
        .map(|i| decoded_digits.get(&i).unwrap())
        .collect::<String>()
        .parse::<u32>()
        .unwrap();

    code
}

pub fn a() {
    let input = read_input();
    let result = input
        .iter()
        .map(|line| infer_and_count_obvious_digits(line))
        .sum::<usize>();
    println!("Day8a: {}", result);
}

pub fn b() {
    let input = read_input();
    let result = input.into_iter().map(|l| solve_line(&l)).sum::<u32>();
    println!("Day8b: {}", result);
}
