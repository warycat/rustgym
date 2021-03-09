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
            if let Op('+') | Op('-') = tok {
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
        let mut lhs = Self::parse_terminal(it)?;
        while let Some(&tok) = it.peek() {
            if let Op('*') | Op('/') = tok {
                let op = *it.next().unwrap();
                let rhs = Self::parse_terminal(it)?;
                lhs = op.eval(lhs, rhs)?;
            } else {
                break;
            }
        }
        Some(lhs)
    }

    fn parse_terminal(it: &mut Peekable<Iter<Tok>>) -> Option<i32> {
        if let Some(Op('(')) = it.peek() {
            it.next();
            let expr = Self::parse_expr(it);
            it.next();
            expr
        } else {
            Self::parse_num(it)
        }
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
                '+' | '-' | '*' | '/' | '(' | ')' => {
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
    let s = "1 + 1".to_string();
    let res = 2;
    assert_eq!(Solution::calculate(s), res);
    let s = " 6-4 / 2 ".to_string();
    let res = 4;
    assert_eq!(Solution::calculate(s), res);
    let s = "2*(5+5*2)/3+(6/2+8)".to_string();
    let res = 21;
    assert_eq!(Solution::calculate(s), res);
    let s = "(2+6* 3+5- (3*14/7+2)*5)+3".to_string();
    let res = -12;
    assert_eq!(Solution::calculate(s), res);
}
