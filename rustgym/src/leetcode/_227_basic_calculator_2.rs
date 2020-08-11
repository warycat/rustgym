struct Solution;

use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tok {
    Num(i32),
    Op(char),
}
use Tok::*;

impl Tok {
    fn is_expr_op(self) -> bool {
        matches!(self, Op('+') | Op('-'))
    }

    fn is_factor_op(self) -> bool {
        matches!(self, Op('*') | Op('/'))
    }

    fn val(self) -> Option<i32> {
        match self {
            Num(x) => Some(x),
            _ => None,
        }
    }
    fn eval(self, lhs: i32, rhs: i32) -> Option<i32> {
        match self {
            Op('+') => Some(lhs + rhs),
            Op('-') => Some(lhs - rhs),
            Op('*') => Some(lhs * rhs),
            Op('/') => {
                if rhs != 0 {
                    Some(lhs / rhs)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Solution {
    fn calculate(s: String) -> i32 {
        let tokens = Self::tokens(&s);
        let mut it = tokens.iter().peekable();
        if let Some(val) = Self::parse_expr(&mut it) {
            val
        } else {
            0
        }
    }

    fn parse_expr(it: &mut Peekable<Iter<Tok>>) -> Option<i32> {
        let mut lhs = Self::parse_factor(it)?;
        while let Some(&tok) = it.peek() {
            if tok.is_expr_op() {
                let op = *it.next().unwrap();
                let rhs = Self::parse_factor(it)?;
                lhs = op.eval(lhs, rhs)?;
            } else {
                break;
            }
        }
        Some(lhs)
    }

    fn parse_factor(it: &mut Peekable<Iter<Tok>>) -> Option<i32> {
        let mut lhs = Self::parse_num(it)?;
        while let Some(&tok) = it.peek() {
            if tok.is_factor_op() {
                let op = *it.next().unwrap();
                let rhs = Self::parse_num(it)?;
                lhs = op.eval(lhs, rhs)?;
            } else {
                break;
            }
        }
        Some(lhs)
    }

    fn parse_num(it: &mut Peekable<Iter<Tok>>) -> Option<i32> {
        match it.next() {
            Some(Tok::Num(x)) => Some(*x),
            _ => None,
        }
    }

    fn tokens(s: &str) -> Vec<Tok> {
        let mut v: Vec<Tok> = vec![];
        let mut it = s.chars().peekable();
        while let Some(c) = it.next() {
            match c {
                '0'..='9' => {
                    let mut x: i32 = (c as u8 - b'0') as i32;
                    while let Some(c) = it.peek() {
                        if c.is_numeric() {
                            x *= 10;
                            x += (it.next().unwrap() as u8 - b'0') as i32;
                        } else {
                            break;
                        }
                    }
                    v.push(Tok::Num(x));
                }
                '+' | '-' | '*' | '/' => {
                    v.push(Tok::Op(c));
                }
                _ => {}
            }
        }
        v
    }
}

#[test]
fn test() {
    let s = "3+2*2".to_string();
    assert_eq!(Solution::calculate(s), 7);
    let s = "0-0".to_string();
    assert_eq!(Solution::calculate(s), 0);
    let s = "0-2147483647".to_string();
    assert_eq!(Solution::calculate(s), -2_147_483_647);
}
