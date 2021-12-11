use std::collections::VecDeque;

use itertools::Itertools;

const INPUT: &str = include_str!("./day10.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChunkType {
    Paren,
    Bracket,
    Brace,
    Angle,
}

#[derive(Debug, Clone, Copy)]
enum Token {
    L(ChunkType),
    R(ChunkType),
}

#[derive(Debug, Clone)]
enum Expression {
    Chunk(ChunkType, Vec<Expression>),
}

fn lex(s: &str) -> Vec<Token> {
    s.chars()
        .map(|c| match c {
            '(' => Token::L(ChunkType::Paren),
            ')' => Token::R(ChunkType::Paren),
            '[' => Token::L(ChunkType::Bracket),
            ']' => Token::R(ChunkType::Bracket),
            '{' => Token::L(ChunkType::Brace),
            '}' => Token::R(ChunkType::Brace),
            '<' => Token::L(ChunkType::Angle),
            '>' => Token::R(ChunkType::Angle),
            _ => panic!("unexpected character: {}", c),
        })
        .collect_vec()
}

#[derive(Debug, Clone)]
enum ParseResult<T> {
    Valid(T),
    Incomplete,
    Corrupted(ChunkType),
}

fn parse(
    tokens: &mut VecDeque<Token>,
    completion: &mut Option<Vec<ChunkType>>,
) -> ParseResult<Option<Expression>> {
    let next = tokens.front().copied();
    let next = match next {
        Some(t) => t,
        None => return ParseResult::Valid(None),
    };

    match next {
        Token::R(_) => return ParseResult::Valid(None),
        Token::L(open_type) => {
            tokens.pop_front().unwrap();

            let mut expressions = Vec::new();

            loop {
                let inner = parse(tokens, completion);

                match inner {
                    ParseResult::Valid(Some(expr)) => expressions.push(expr),
                    ParseResult::Valid(None) => {}
                    ParseResult::Corrupted(ch) => {
                        return ParseResult::Corrupted(ch);
                    }
                    ParseResult::Incomplete => {
                        if let Some(completion) = completion {
                            completion.push(open_type);
                        }

                        return ParseResult::Incomplete;
                    }
                }

                let next = tokens.front().copied();
                match next {
                    Some(Token::R(ch)) if ch == open_type => {
                        break;
                    }
                    Some(Token::R(ch)) => {
                        return ParseResult::Corrupted(ch);
                    }
                    Some(Token::L(_)) => {}
                    None => {
                        if let Some(completion) = completion {
                            completion.push(open_type);
                        }

                        return ParseResult::Incomplete;
                    }
                }
            }

            tokens.pop_front().unwrap();

            ParseResult::Valid(Some(Expression::Chunk(open_type, expressions)))
        }
    }
}

fn parse_line(line: &str, completion: &mut Option<Vec<ChunkType>>) -> ParseResult<Vec<Expression>> {
    let mut tokens = VecDeque::from(lex(line));
    let mut expressions: Vec<Expression> = Vec::new();

    loop {
        let next = parse(&mut tokens, completion);
        match next {
            ParseResult::Valid(Some(expr)) => expressions.push(expr),
            ParseResult::Valid(None) => break,
            ParseResult::Incomplete => {
                return ParseResult::Incomplete;
            }
            ParseResult::Corrupted(ch) => {
                return ParseResult::Corrupted(ch);
            }
        }
    }

    match tokens.front() {
        None => ParseResult::Valid(expressions),
        Some(Token::R(ch)) => ParseResult::Corrupted(*ch),
        Some(Token::L(_)) => unreachable!(),
    }
}

pub fn a() {
    let mut score = 0;

    for line in INPUT.lines() {
        let parsed = parse_line(line, &mut None);
        let value = match parsed {
            ParseResult::Corrupted(ChunkType::Paren) => 3,
            ParseResult::Corrupted(ChunkType::Bracket) => 57,
            ParseResult::Corrupted(ChunkType::Brace) => 1197,
            ParseResult::Corrupted(ChunkType::Angle) => 25137,
            _ => 0,
        };

        score += value;
    }

    println!("Day10a: {}", score);
}

fn score_completion(completion: &[ChunkType]) -> usize {
    let mut score = 0;

    for ch in completion.iter().copied() {
        score *= 5;

        score += match ch {
            ChunkType::Paren => 1,
            ChunkType::Bracket => 2,
            ChunkType::Brace => 3,
            ChunkType::Angle => 4,
        };
    }

    score
}

pub fn b() {
    let mut scores = Vec::new();

    for line in INPUT.lines() {
        let mut completion = Some(Vec::new());
        parse_line(line, &mut completion);
        let value = score_completion(completion.as_ref().unwrap());

        if value > 0 {
            scores.push(value);
        }
    }

    scores.sort_unstable();

    let middle_score = scores[scores.len() / 2];

    println!("Day10b: {}", middle_score);
}
