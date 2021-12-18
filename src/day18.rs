// This would have been impossible without
// https://www.reddit.com/r/adventofcode/comments/rj1p92/2021_day_18_part_1_if_i_encounter_a_pair_that/

use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
};

use itertools::Itertools;

const INPUT: &str = include_str!("./day18.txt");

trait ParseExt<T> {
    fn expect_and_pop(&mut self, token: T);
}

impl ParseExt<Token> for VecDeque<Token> {
    fn expect_and_pop(&mut self, token: Token) {
        match self.front().copied() {
            Some(t) if t == token => {
                self.pop_front().unwrap();
            }
            Some(t) => panic!("expected {:?} but got {:?}", token, t),
            None => panic!("unexpected end of input"),
        }
    }
}

type Literal = u8;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pair(Tree, Tree);

#[derive(Clone, PartialEq, Eq)]
enum Tree {
    Literal(Literal),
    Pair(Box<Pair>),
}

impl Tree {
    fn new_pair(l: impl Into<Tree>, r: impl Into<Tree>) -> Tree {
        Tree::Pair(Box::new(Pair(l.into(), r.into())))
    }

    fn unwrap_pair(self) -> Pair {
        match self {
            Tree::Pair(pair) => *pair,
            _ => panic!("expected pair"),
        }
    }

    fn as_pair(&self) -> Option<&Pair> {
        match self {
            Tree::Pair(pair) => Some(pair),
            _ => None,
        }
    }

    fn as_literal(&self) -> Option<Literal> {
        match self {
            Tree::Literal(literal) => Some(*literal),
            _ => None,
        }
    }
}

impl From<Literal> for Tree {
    fn from(literal: Literal) -> Self {
        Tree::Literal(literal)
    }
}

impl From<Pair> for Tree {
    fn from(pair: Pair) -> Self {
        Tree::Pair(Box::new(pair))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    LBracket,
    RBracket,
    Comma,
    Literal(Literal),
}

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tree::Literal(literal) => write!(f, "{}", literal),
            Tree::Pair(pair) => write!(f, "[{}, {}]", pair.0, pair.1),
        }
    }
}

impl std::fmt::Debug for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

fn parse_line(input: &str) -> Tree {
    let mut tokens = input
        .chars()
        .filter_map(|ch| match ch {
            '[' => Some(Token::LBracket),
            ']' => Some(Token::RBracket),
            ',' => Some(Token::Comma),
            ' ' => None,
            _ => Some(Token::Literal(ch.to_digit(10).unwrap() as Literal)),
        })
        .collect();

    fn parse_pair(input: &mut VecDeque<Token>) -> Tree {
        input.expect_and_pop(Token::LBracket);

        let l = parse_tree(input);

        input.expect_and_pop(Token::Comma);

        let r = parse_tree(input);

        input.expect_and_pop(Token::RBracket);

        Tree::Pair(Box::new(Pair(l, r)))
    }

    fn parse_tree(input: &mut VecDeque<Token>) -> Tree {
        match input.front().copied() {
            Some(Token::LBracket) => parse_pair(input),
            Some(Token::Literal(literal)) => {
                input.pop_front().unwrap();
                let mut literal = literal.to_string();

                while let Some(Token::Literal(digit)) = input.front().copied() {
                    input.pop_front();
                    literal += &digit.to_string();
                }

                Tree::Literal(literal.parse().unwrap())
            }
            Some(other) => panic!("expected literal or '[' but got {:?}", other),
            None => panic!("unexpected end of input"),
        }
    }

    let pair = parse_pair(&mut tokens);

    assert_eq!(tokens.len(), 0);

    pair
}

fn add_tree(a: Tree, b: Tree) -> Tree {
    Tree::Pair(Box::new(Pair(a, b)))
}

fn add_and_reduce(a: Tree, b: Tree) -> Tree {
    let acc = add_tree(a, b);
    // println!("after addition:  {}", acc);
    reduce_all(acc)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Action {
    Explosion,
    Split,
}

enum ReductionResult {
    Unchanged(Tree),
    Reduced(Tree, Action),
    Exploding {
        tree: Tree,
        left: Option<u8>,
        right: Option<u8>,
    },
}

fn add_to_lefmost(tree: Tree, value: Literal) -> Tree {
    match tree {
        Tree::Literal(literal) => Tree::Literal(literal + value),
        Tree::Pair(pair) => Tree::new_pair(add_to_lefmost(pair.0, value), pair.1),
    }
}

fn add_to_rightmost(tree: Tree, value: Literal) -> Tree {
    match tree {
        Tree::Literal(literal) => Tree::Literal(literal + value),
        Tree::Pair(pair) => Tree::new_pair(pair.0, add_to_rightmost(pair.1, value)),
    }
}

fn reduce_pair(pair: Pair, depth: usize, action: Action) -> ReductionResult {
    match pair {
        Pair(Tree::Literal(l), Tree::Literal(r)) if action == Action::Explosion && depth >= 4 => {
            ReductionResult::Exploding {
                tree: Tree::Literal(0),
                left: Some(l),
                right: Some(r),
            }
        }
        Pair(left, right) => {
            let left_reduction = match left {
                Tree::Pair(l_pair) => reduce_pair(*l_pair, depth + 1, action),
                literal_tree => ReductionResult::Unchanged(literal_tree),
            };

            let left = match left_reduction {
                ReductionResult::Unchanged(left) => left,
                ReductionResult::Reduced(left, action) => {
                    return ReductionResult::Reduced(Tree::new_pair(left, right), action);
                }
                ReductionResult::Exploding {
                    tree: left,
                    left: left_exploded,
                    right: right_exploded,
                } => match (left_exploded, right_exploded, right) {
                    (None, None, right) => {
                        return ReductionResult::Reduced(
                            Tree::new_pair(left, right),
                            Action::Explosion,
                        );
                    }
                    (left_exploded, None, right) => {
                        return ReductionResult::Exploding {
                            tree: Tree::new_pair(left, right),
                            left: left_exploded,
                            right: None,
                        };
                    }
                    (left_exploded, Some(exploded_right), right) => {
                        return ReductionResult::Exploding {
                            tree: Tree::new_pair(left, add_to_lefmost(right, exploded_right)),
                            left: left_exploded,
                            right: None,
                        }
                    }
                },
            };

            if action == Action::Split {
                if let Some(n) = left.as_literal() {
                    if n >= 10 {
                        let l_div: f32 = n as f32 / 2.0;
                        let l_l = l_div.floor() as Literal;
                        let l_r = l_div.ceil() as Literal;

                        return ReductionResult::Reduced(
                            Tree::new_pair(Tree::new_pair(l_l, l_r), right),
                            Action::Split,
                        );
                    }
                }
            }

            let right_reduction = match right {
                Tree::Pair(r_pair) => reduce_pair(*r_pair, depth + 1, action),
                literal_tree => ReductionResult::Unchanged(literal_tree),
            };

            match right_reduction {
                ReductionResult::Unchanged(right) => {
                    if action == Action::Split {
                        if let Some(n) = right.as_literal() {
                            if n >= 10 {
                                let r_div: f32 = n as f32 / 2.0;
                                let r_l = r_div.floor() as Literal;
                                let r_r = r_div.ceil() as Literal;

                                return ReductionResult::Reduced(
                                    Tree::new_pair(left, Tree::new_pair(r_l, r_r)),
                                    Action::Split,
                                );
                            }
                        }
                    }

                    ReductionResult::Unchanged(Tree::new_pair(left, right))
                }
                ReductionResult::Reduced(right, action) => {
                    ReductionResult::Reduced(Tree::new_pair(left, right), action)
                }
                ReductionResult::Exploding {
                    tree: right,
                    left: left_exploded,
                    right: right_exploded,
                } => match (left, left_exploded, right, right_exploded) {
                    (left, None, right, None) => {
                        ReductionResult::Reduced(Tree::new_pair(left, right), Action::Explosion)
                    }
                    (left, Some(left_exploded), right, right_exploded) => {
                        ReductionResult::Exploding {
                            tree: Tree::new_pair(add_to_rightmost(left, left_exploded), right),
                            left: None,
                            right: right_exploded,
                        }
                    }
                    (left, None, right, right_exploded) => ReductionResult::Exploding {
                        tree: Tree::new_pair(left, right),
                        left: None,
                        right: right_exploded,
                    },
                },
            }
        }
    }
}

fn reduce_with_action(pair: Pair, only_action: Action) -> Tree {
    match reduce_pair(pair, 0, only_action) {
        ReductionResult::Unchanged(pair) => pair,
        ReductionResult::Reduced(Tree::Pair(pair), action) => {
            match action {
                Action::Explosion => {
                    // println!("after explosion: {}", Tree::Pair(pair.clone()));
                }
                Action::Split => {
                    // println!("after split:     {}", Tree::Pair(pair.clone()));
                }
            }
            reduce_with_action(*pair, only_action)
        }
        ReductionResult::Exploding {
            tree: Tree::Pair(pair),
            ..
        } => {
            // println!("after explosion: {}", Tree::Pair(pair.clone()));
            reduce_with_action(*pair, only_action)
        }
        _ => unreachable!(),
    }
}

fn reduce_once_with_action(pair: Pair, only_action: Action) -> Tree {
    match reduce_pair(pair, 0, only_action) {
        ReductionResult::Unchanged(pair) => pair,
        ReductionResult::Reduced(Tree::Pair(pair), action) => {
            match action {
                Action::Explosion => {
                    // println!("after explosion: {}", Tree::Pair(pair.clone()));
                }
                Action::Split => {
                    // println!("after split:     {}", Tree::Pair(pair.clone()));
                }
            }
            Tree::Pair(pair)
        }
        ReductionResult::Exploding {
            tree: Tree::Pair(pair),
            ..
        } => {
            // println!("after explosion: {}", Tree::Pair(pair.clone()));
            Tree::Pair(pair)
        }
        _ => unreachable!(),
    }
}

fn reduce_all(mut acc: Tree) -> Tree {
    loop {
        let exploded = reduce_with_action(acc.clone().unwrap_pair(), Action::Explosion);
        let split = reduce_once_with_action(exploded.unwrap_pair(), Action::Split);

        if split == acc {
            return split;
        } else {
            acc = split;
        }
    }
}

fn sum_and_reduce(mut trees: Vec<Tree>) -> Tree {
    let mut acc = trees.remove(0);

    for tree in trees {
        acc = add_and_reduce(acc, tree);
    }

    acc
}

fn magnitude(tree: Tree) -> usize {
    match tree {
        Tree::Literal(n) => n as usize,
        Tree::Pair(pair) => {
            let Pair(left, right) = *pair;
            magnitude(left) * 3 + magnitude(right) * 2
        }
    }
}

pub fn ab() {
    let input = INPUT.lines().map(parse_line).collect_vec();
    let total = sum_and_reduce(input.clone());
    println!("Day18a: {}", magnitude(total));

    let mut largest_magnitude = usize::MIN;

    for pair in input.into_iter().permutations(2) {
        let a = pair[0].clone();
        let b = pair[1].clone();
        let sum = add_and_reduce(a, b);
        let total_magnitude = magnitude(sum);

        largest_magnitude = largest_magnitude.max(total_magnitude);
    }

    println!("Day18b: {}", largest_magnitude);
}

#[cfg(test)]
mod test {
    use super::*;

    fn reduce_once(pair: Pair, action: Action) -> Tree {
        match reduce_pair(pair, 0, action) {
            ReductionResult::Unchanged(pair) => pair,
            ReductionResult::Reduced(pair, _) => pair,
            ReductionResult::Exploding { tree: pair, .. } => pair,
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_addition() {
        let a = parse_line("[1,2]");
        let b = parse_line("[[3,4],5]");
        let expected = parse_line("[[1,2],[[3,4],5]]");
        assert_eq!(add_tree(a, b), expected);
    }

    #[test]
    fn test_explosion_a() {
        let a = parse_line("[[[[[9,8],1],2],3],4]");
        let expected = parse_line("[[[[0,9],2],3],4]");
        let exploded = reduce_once(a.unwrap_pair(), Action::Explosion);
        assert_eq!(exploded, expected);
    }

    #[test]
    fn test_explosion_b() {
        let a = parse_line("[7,[6,[5,[4,[3,2]]]]]");
        let expected = parse_line("[7,[6,[5,[7,0]]]]");
        let exploded = reduce_once(a.unwrap_pair(), Action::Explosion);
        assert_eq!(exploded, expected);
    }

    #[test]
    fn test_explosion_c() {
        let a = parse_line("[[6,[5,[4,[3,2]]]],1]");
        let expected = parse_line("[[6,[5,[7,0]]],3]");
        let exploded = reduce_once(a.unwrap_pair(), Action::Explosion);
        assert_eq!(exploded, expected);
    }

    #[test]
    fn test_explosion_d() {
        let a = parse_line("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        let expected = parse_line("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        let exploded = reduce_once(a.unwrap_pair(), Action::Explosion);
        assert_eq!(exploded, expected);
    }

    #[test]
    fn test_explosion_e() {
        let a = parse_line("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        let expected = parse_line("[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
        let exploded = reduce_once(a.unwrap_pair(), Action::Explosion);
        assert_eq!(exploded, expected);
    }

    #[test]
    fn test_split_a() {
        let a = Tree::new_pair(10, 0);
        let expected = parse_line("[[5, 5], 0]");
        assert_eq!(reduce_once(a.unwrap_pair(), Action::Split), expected);
    }

    #[test]
    fn full_reduction_a() {
        let a = parse_line("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let b = parse_line("[1,1]");
        let reduced = add_and_reduce(a, b);
        let expected = parse_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(reduced, expected);
    }

    #[test]
    fn add_all_a() {
        let input = vec![
            parse_line("[1,1]"),
            parse_line("[2,2]"),
            parse_line("[3,3]"),
            parse_line("[4,4]"),
        ];

        let reduced = sum_and_reduce(input);
        let expected = parse_line("[[[[1,1],[2,2]],[3,3]],[4,4]]");

        assert_eq!(reduced, expected);
    }

    #[test]
    fn add_all_b() {
        let input = vec![
            parse_line("[1,1]"),
            parse_line("[2,2]"),
            parse_line("[3,3]"),
            parse_line("[4,4]"),
            parse_line("[5,5]"),
        ];

        let reduced = sum_and_reduce(input);
        let expected = parse_line("[[[[3,0],[5,3]],[4,4]],[5,5]]");

        assert_eq!(reduced, expected);
    }

    #[test]
    fn add_all_c() {
        let input = vec![
            parse_line("[1,1]"),
            parse_line("[2,2]"),
            parse_line("[3,3]"),
            parse_line("[4,4]"),
            parse_line("[5,5]"),
            parse_line("[6,6]"),
        ];

        let reduced = sum_and_reduce(input);
        let expected = parse_line("[[[[5,0],[7,4]],[5,5]],[6,6]]");

        assert_eq!(reduced, expected);
    }

    #[test]
    fn add_all_d() {
        let input = vec![
            parse_line("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"),
            parse_line("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"),
            parse_line("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]"),
            parse_line("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]"),
            parse_line("[7,[5,[[3,8],[1,4]]]]"),
            parse_line("[[2,[2,2]],[8,[8,1]]]"),
            parse_line("[2,9]"),
            parse_line("[1,[[[9,3],9],[[9,0],[0,7]]]]"),
            parse_line("[[[5,[7,4]],7],1]"),
            parse_line("[[[[4,2],2],6],[8,7]]"),
        ];

        let reduced = sum_and_reduce(input);
        let expected = parse_line("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");

        assert_eq!(reduced, expected);
    }

    #[test]
    fn add_all_d_part_1() {
        let a = add_and_reduce(
            parse_line("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"),
            parse_line("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"),
        );
        let expected = parse_line("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
        assert_eq!(a, expected);
    }

    #[test]
    fn add_all_d_part_3() {
        let a = add_and_reduce(
            parse_line("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"),
            parse_line("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]"),
        );
        let expected = parse_line("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]");
        assert_eq!(a, expected);
    }

    #[test]
    fn add_all_d_part_4() {
        let a = add_and_reduce(
            parse_line("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]"),
            parse_line("[7,[5,[[3,8],[1,4]]]]"),
        );
        let expected = parse_line("[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]");
        assert_eq!(a, expected);
    }

    #[test]
    fn add_all_d_part_4_isolated() {
        let a = parse_line("[[[[7,7],[7,8]],[[9,5],[8,0]]],[[[9,10],20],[8,[9,0]]]]");
        let expected = parse_line("[[[[7,7],[7,8]],[[9,5],[8,0]]],[[[9,[5,5]],20],[8,[9,0]]]]");
        let result = reduce_once_with_action(a.unwrap_pair(), Action::Split);
        assert_eq!(result, expected);
    }

    #[test]
    fn add_all_d_part_4_isolated_more() {
        let a = parse_line("[[9,10], 20]");
        let expected = parse_line("[[9, [5, 5]], 20]");
        let result = reduce_once_with_action(a.unwrap_pair(), Action::Split);
        assert_eq!(result, expected);
    }

    #[test]
    fn split_two() {
        let a = Tree::new_pair(10, 20);
        let expected_a = Tree::new_pair(Tree::new_pair(5, 5), 20);
        let result = reduce_once_with_action(a.unwrap_pair(), Action::Split);
        assert_eq!(result, expected_a);
        let expected_b = Tree::new_pair(Tree::new_pair(5, 5), Tree::new_pair(10, 10));
        let result = reduce_once_with_action(result.unwrap_pair(), Action::Split);
        assert_eq!(result, expected_b);
        let expected_c = Tree::new_pair(
            Tree::new_pair(5, 5),
            Tree::new_pair(Tree::new_pair(5, 5), 10),
        );
        let result = reduce_once_with_action(result.unwrap_pair(), Action::Split);
        assert_eq!(result, expected_c);
        let expected_d = Tree::new_pair(
            Tree::new_pair(5, 5),
            Tree::new_pair(Tree::new_pair(5, 5), Tree::new_pair(5, 5)),
        );
        let result = reduce_once_with_action(result.unwrap_pair(), Action::Split);
        assert_eq!(result, expected_d);
    }

    #[test]
    fn magnitude_a() {
        assert_eq!(magnitude(parse_line("[[1,2],[[3,4],5]]")), 143)
    }
}
