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
    i: &mut usize,
    line: usize,
) -> ParseResult<Option<Expression>> {
    let next = tokens.front().copied();
    let next = match next {
        Some(t) => t,
        None => return ParseResult::Valid(None),
    };
    // println!("Peeked at token: {:?}", next);
    *i += 1;

    match next {
        Token::R(_) => return ParseResult::Valid(None),
        Token::L(open_type) => {
            let _popped = tokens.pop_front().unwrap();
            // println!("Popped start token {:?}", popped);

            let mut expressions = Vec::new();

            loop {
                let inner = parse(tokens, i, line);

                match inner {
                    ParseResult::Valid(Some(expr)) => expressions.push(expr),
                    ParseResult::Valid(None) => {
                        break;
                    }
                    ParseResult::Incomplete => {
                        return ParseResult::Incomplete;
                    }
                    ParseResult::Corrupted(ch) => {
                        return ParseResult::Corrupted(ch);
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
                        break;
                    }
                }
            }

            while let ParseResult::Valid(Some(expr)) = parse(tokens, i, line) {
                expressions.push(expr);
            }

            // println!("subtree {:?}", expressions);

            let end_token = tokens.pop_front();
            // println!("Popped end token {:?}", end_token);
            *i += 1;

            match end_token {
                Some(Token::R(close_type)) => {
                    if close_type != open_type {
                        ParseResult::Corrupted(close_type)
                    } else {
                        ParseResult::Valid(Some(Expression::Chunk(open_type, expressions)))
                    }
                }
                Some(token) => {
                    panic!(
                        "unexpected token {:?} at {}:{}, expected closing chunk of type {:?}",
                        token, line, i, open_type
                    );
                }
                None => return ParseResult::Incomplete,
            }
        }
    }
}

fn parse_line(line: &str, line_number: usize) -> ParseResult<Vec<Expression>> {
    let mut tokens = VecDeque::from(lex(line));
    let mut i = 0;

    let mut expressions: Vec<Expression> = Vec::new();

    println!("{}", line);

    loop {
        let next = parse(&mut tokens, &mut i, line_number);
        match next {
            ParseResult::Valid(Some(expr)) => expressions.push(expr),
            ParseResult::Valid(None) => break,
            ParseResult::Incomplete => return ParseResult::Incomplete,
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

    for (line_number, line) in INPUT.lines().enumerate() {
        let parsed = parse_line(line, line_number);
        let value = match parsed {
            ParseResult::Corrupted(ChunkType::Paren) => 3,
            ParseResult::Corrupted(ChunkType::Bracket) => 57,
            ParseResult::Corrupted(ChunkType::Brace) => 1197,
            ParseResult::Corrupted(ChunkType::Angle) => 25137,
            _ => 0,
        };

        score += value;

        println!("{:?}", parsed);
    }

    println!("score: {}", score);
}
