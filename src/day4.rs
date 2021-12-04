use itertools::Itertools;
use retain_mut::RetainMut;

use crate::util::BoolIterUtil;

const INPUT: &str = include_str!("./day4.txt");

type Board = [[(u32, bool); 5]; 5];

fn parse_input() -> (Vec<u32>, Vec<Board>) {
    let mut lines = INPUT.lines();

    let numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let boards = lines
        .filter(|&line| line != "")
        .chunks(5)
        .into_iter()
        .map(|chunk| {
            let board: Board = chunk
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|number| (number.parse().unwrap(), false))
                        .collect_vec()
                        .try_into()
                        .unwrap()
                })
                .collect_vec()
                .try_into()
                .unwrap();
            board
        })
        .collect_vec();

    (numbers, boards)
}

fn mark_number(number: u32, board: &mut Board) {
    for row in board {
        for (n, marked) in row {
            if *n == number {
                *marked = true;
                return;
            }
        }
    }
}

fn check_bingo(board: &Board) -> bool {
    let any_row = board
        .iter()
        .map(|row| row.iter().map(|(_, marked)| marked).all_true())
        .any(|id| id);

    if any_row {
        return true;
    }

    (0..5)
        .map(|col| (0..5).map(|row| board[row][col].1).all_true())
        .any_true()
}

fn unmarked_numbers(board: &Board) -> Vec<u32> {
    let mut numbers = vec![];
    for row in board {
        for (n, marked) in row {
            if !*marked {
                numbers.push(*n);
            }
        }
    }
    numbers
}

pub fn a() {
    let (numbers, mut boards) = parse_input();

    for number in numbers {
        for board in boards.iter_mut() {
            mark_number(number, board);
            if check_bingo(board) {
                let sum_of_unmarked: u32 = unmarked_numbers(board).into_iter().sum();
                println!("Day4a: {}", sum_of_unmarked * number);
                return;
            }
        }
    }

    println!("No solution found :(");
}

pub fn b() {
    let (numbers, mut boards) = parse_input();

    while boards.len() > 0 {
        for number in &numbers {
            let boards_len = boards.len();
            let mut found_winner = false;

            boards.retain_mut(|board| {
                mark_number(*number, board);
                let found_bingo = check_bingo(board);

                if found_bingo {
                    if boards_len > 1 {
                        false
                    } else {
                        found_winner = true;
                        true
                    }
                } else {
                    true
                }
            });

            if boards.len() == 1 && found_winner {
                let sum_of_unmarked: u32 = unmarked_numbers(&boards[0]).into_iter().sum();
                println!("Day4b: {}", sum_of_unmarked * number);
                return;
            }
        }
    }

    println!("No solution found :(");
}
