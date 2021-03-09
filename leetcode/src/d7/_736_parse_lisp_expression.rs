struct Solution;

use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;
use std::vec::IntoIter;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tok {
    Var(usize),
    Num(i32),
    Op(char),
}

use Tok::*;

impl Solution {
    fn evaluate(expression: String) -> i32 {
        let mut it = expression.chars().peekable();
        let mut symbols: HashMap<String, usize> = HashMap::new();
        let tokens = Self::parse_tokens(&mut it, &mut symbols);
        let mut it = tokens.into_iter().peekable();
        let k = symbols.len();
        let mut stacks = vec![vec![]; k];
        Self::eval_expr(&mut it, &mut stacks).unwrap()
    }

    fn eval_expr(it: &mut Peekable<IntoIter<Tok>>, stacks: &mut Vec<Vec<i32>>) -> Option<i32> {
        let tok = it.next().unwrap();
        match tok {
            Op('(') => {
                let res = Self::eval_complex_expr(it, stacks).unwrap();
                let close = it.next().unwrap();
                if close != Op(')') {
                    panic!();
                }
                Some(res)
            }
            Var(id) => Some(stacks[id].last().copied().unwrap()),
            Num(x) => Some(x),
            _ => {
                panic!()
            }
        }
    }

    fn eval_complex_expr(
        it: &mut Peekable<IntoIter<Tok>>,
        stacks: &mut Vec<Vec<i32>>,
    ) -> Option<i32> {
        let tok = it.next().unwrap();
        match tok {
            Op('=') => {
                let mut locals = vec![];
                let mut ambiguous: Option<i32> = None;
                while let Some(Var(_)) = it.peek() {
                    if let Some(Var(id)) = it.next() {
                        if let Some(Op(')')) = it.peek() {
                            let val = stacks[id].last().copied().unwrap();
                            ambiguous = Some(val);
                            break;
                        } else {
                            let expr = Self::eval_expr(it, stacks).unwrap();
                            stacks[id].push(expr);
                            locals.push(id);
                        }
                    } else {
                        panic!();
                    }
                }
                let res = if let Some(val) = ambiguous {
                    val
                } else {
                    Self::eval_expr(it, stacks).unwrap()
                };
                for id in locals {
                    stacks[id].pop().unwrap();
                }
                Some(res)
            }
            Op('+') => {
                Some(Self::eval_expr(it, stacks).unwrap() + Self::eval_expr(it, stacks).unwrap())
            }
            Op('*') => {
                Some(Self::eval_expr(it, stacks).unwrap() * Self::eval_expr(it, stacks).unwrap())
            }
            _ => panic!(),
        }
    }

    fn parse_tokens(it: &mut Peekable<Chars>, symbols: &mut HashMap<String, usize>) -> Vec<Tok> {
        let mut res = vec![];
        while let Some(c) = it.next() {
            match c {
                '(' | ')' => res.push(Tok::Op(c)),
                '-' => {
                    let mut x = 0;
                    while let Some(next_c) = it.peek() {
                        if next_c.is_numeric() {
                            x *= 10;
                            x += (it.next().unwrap() as u8 - b'0') as i32;
                        } else {
                            break;
                        }
                    }
                    res.push(Tok::Num(-x));
                }
                '0'..='9' => {
                    let mut x = (c as u8 - b'0') as i32;
                    while let Some(next_c) = it.peek() {
                        if next_c.is_numeric() {
                            x *= 10;
                            x += (it.next().unwrap() as u8 - b'0') as i32;
                        } else {
                            break;
                        }
                    }
                    res.push(Tok::Num(x));
                }
                'a'..='z' => {
                    let mut s = "".to_string();
                    s.push(c);
                    while let Some(next_c) = it.peek() {
                        if next_c.is_alphanumeric() {
                            s.push(it.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    match s.as_str() {
                        "let" => res.push(Tok::Op('=')),
                        "mult" => res.push(Tok::Op('*')),
                        "add" => res.push(Tok::Op('+')),
                        _ => {
                            let size = symbols.len();
                            let id = *symbols.entry(s).or_insert(size);
                            res.push(Tok::Var(id));
                        }
                    }
                }
                ' ' => {}
                _ => panic!(),
            }
        }
        res
    }
}

#[test]
fn test() {
    let expression = "(add 1 2)".to_string();
    let res = 3;
    assert_eq!(Solution::evaluate(expression), res);
    let expression = "(mult 3 (add 2 3))".to_string();
    let res = 15;
    assert_eq!(Solution::evaluate(expression), res);
    let expression = "(let x 2 (mult x 5))".to_string();
    let res = 10;
    assert_eq!(Solution::evaluate(expression), res);
    let expression = "(let x 2 (mult x (let x 3 y 4 (add x y))))".to_string();
    let res = 14;
    assert_eq!(Solution::evaluate(expression), res);
    let expression = "(let x 3 x 2 x)".to_string();
    let res = 2;
    assert_eq!(Solution::evaluate(expression), res);
    let expression = "(let x 1 y 2 x (add x y) (add x y))".to_string();
    let res = 5;
    assert_eq!(Solution::evaluate(expression), res);
    let expression = "(let x 2 (add (let x 3 (let x 4 x)) x))".to_string();
    let res = 6;
    assert_eq!(Solution::evaluate(expression), res);
    let expression = "(let a1 3 b2 (add a1 1) b2)".to_string();
    let res = 4;
    assert_eq!(Solution::evaluate(expression), res);
    let expression = "(let x 7 -12)".to_string();
    let res = -12;
    assert_eq!(Solution::evaluate(expression), res);
}
