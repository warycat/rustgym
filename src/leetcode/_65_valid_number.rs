struct Solution;

use std::cmp::Ordering::*;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tok {
    Num(i32),
    Op(char),
}

impl Solution {
    fn is_number(s: String) -> bool {
        let s = s.trim();
        let mut it = s.chars().peekable();
        let mut tokens: Vec<Tok> = vec![];
        let mut e_count = 0;
        while let Some(c) = it.next() {
            match c {
                ' ' | '-' | '+' | '.' => {
                    tokens.push(Tok::Op(c));
                }
                'e' => {
                    e_count += 1;
                    if e_count == 2 {
                        return false;
                    }
                    tokens.push(Tok::Op(c));
                }
                '0'..='9' => {
                    let mut x = (c as u8 - b'0') as i32;
                    while let Some('0'..='9') = it.peek() {
                        x *= 10;
                        x += (it.next().unwrap() as u8 - b'0') as i32;
                    }
                    tokens.push(Tok::Num(x));
                }
                _ => {
                    return false;
                }
            }
        }

        if let Some(e_pos) = tokens.iter().position(|&x| x == Tok::Op('e')) {
            let mut left = vec![];
            let mut right = vec![];
            for (i, tok) in tokens.into_iter().enumerate() {
                match i.cmp(&e_pos) {
                    Less => {
                        left.push(tok);
                    }
                    Greater => {
                        right.push(tok);
                    }
                    _ => {}
                }
            }
            Self::parse_float(&mut left.into_iter().peekable())
                && Self::parse_int(&mut right.into_iter().peekable())
        } else {
            Self::parse_float(&mut tokens.into_iter().peekable())
        }
    }

    fn parse_int(it: &mut Peekable<IntoIter<Tok>>) -> bool {
        if let Some(Tok::Op('+')) | Some(Tok::Op('-')) = it.peek() {
            it.next();
        }
        Self::parse_uint(it)
    }

    fn parse_uint(it: &mut Peekable<IntoIter<Tok>>) -> bool {
        if let Some(Tok::Num(_)) = it.next() {
            it.next().is_none()
        } else {
            false
        }
    }

    fn parse_ufloat(it: &mut Peekable<IntoIter<Tok>>) -> bool {
        match it.peek() {
            Some(Tok::Op('.')) => {
                it.next();
                Self::parse_uint(it)
            }
            Some(Tok::Num(_)) => {
                it.next();
                match it.peek() {
                    Some(Tok::Op('.')) => {
                        it.next();
                        if it.peek().is_none() {
                            true
                        } else {
                            Self::parse_uint(it)
                        }
                    }
                    None => true,
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn parse_float(it: &mut Peekable<IntoIter<Tok>>) -> bool {
        if let Some(Tok::Op('+')) | Some(Tok::Op('-')) = it.peek() {
            it.next();
        }

        Self::parse_ufloat(it)
    }
}

#[test]
fn test() {
    let s = "0".to_string();
    let res = true;
    assert_eq!(Solution::is_number(s), res);
    let s = " 0.1 ".to_string();
    let res = true;
    assert_eq!(Solution::is_number(s), res);
    let s = "abc".to_string();
    let res = false;
    assert_eq!(Solution::is_number(s), res);
    let s = "1 a".to_string();
    let res = false;
    assert_eq!(Solution::is_number(s), res);
    let s = "2e10".to_string();
    let res = true;
    assert_eq!(Solution::is_number(s), res);
    let s = " -90e3".to_string();
    let res = true;
    assert_eq!(Solution::is_number(s), res);
    let s = " 1e".to_string();
    let res = false;
    assert_eq!(Solution::is_number(s), res);
    let s = "e3".to_string();
    let res = false;
    assert_eq!(Solution::is_number(s), res);
    let s = " 6e-1".to_string();
    let res = true;
    assert_eq!(Solution::is_number(s), res);
    let s = " 99e2.5".to_string();
    let res = false;
    assert_eq!(Solution::is_number(s), res);
    let s = " 53.5e93".to_string();
    let res = true;
    assert_eq!(Solution::is_number(s), res);
    let s = " --6 ".to_string();
    let res = false;
    assert_eq!(Solution::is_number(s), res);
    let s = "-+3".to_string();
    let res = false;
    assert_eq!(Solution::is_number(s), res);
    let s = "95a54e53".to_string();
    let res = false;
    assert_eq!(Solution::is_number(s), res);
    let s = ".1".to_string();
    let res = true;
    assert_eq!(Solution::is_number(s), res);
    let s = "3.".to_string();
    let res = true;
    assert_eq!(Solution::is_number(s), res);
    let s = ". 1".to_string();
    let res = false;
    assert_eq!(Solution::is_number(s), res);
}
