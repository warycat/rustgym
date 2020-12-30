struct Solution;

use std::iter::Peekable;
use std::str::Chars;
use std::vec::IntoIter;

#[derive(Debug)]
enum Tok {
    Num(i32),
    Op(char),
}

impl Solution {
    fn calculate(s: String) -> i32 {
        let mut it = s.chars().peekable();
        let tokens = Self::parse_tokens(&mut it);
        let mut it = tokens.into_iter().peekable();
        Self::parse(&mut it)
    }

    fn parse_term(it: &mut Peekable<IntoIter<Tok>>) -> i32 {
        if Self::eat(it, '(') {
            let res = Self::parse(it);
            Self::eat(it, ')');
            res
        } else {
            if let Some(Tok::Num(x)) = it.next() {
                x
            } else {
                panic!()
            }
        }
    }

    fn parse(it: &mut Peekable<IntoIter<Tok>>) -> i32 {
        let mut left = Self::parse_term(it);
        loop {
            if Self::eat(it, '+') {
                left += Self::parse_term(it);
                continue;
            }
            if Self::eat(it, '-') {
                left -= Self::parse_term(it);
                continue;
            }
            break;
        }
        left
    }

    fn eat(it: &mut Peekable<IntoIter<Tok>>, c: char) -> bool {
        if let Some(&Tok::Op(first)) = it.peek() {
            if first == c {
                it.next();
                return true;
            }
        }
        false
    }

    fn parse_tokens(it: &mut Peekable<Chars>) -> Vec<Tok> {
        let mut res = vec![];
        while let Some(c) = it.next() {
            match c {
                '0'..='9' => {
                    let mut x = (c as u8 - b'0') as i32;
                    while let Some('0'..='9') = it.peek() {
                        x *= 10;
                        x += (it.next().unwrap() as u8 - b'0') as i32;
                    }
                    res.push(Tok::Num(x));
                }
                '(' | ')' | '+' | '-' => {
                    res.push(Tok::Op(c));
                }
                _ => {}
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "1 + 1".to_string();
    let res = 2;
    assert_eq!(Solution::calculate(s), res);
    let s = " 2-1 + 2 ".to_string();
    let res = 3;
    assert_eq!(Solution::calculate(s), res);
    let s = "(1+(4+5+2)-3)+(6+8)".to_string();
    let res = 23;
    assert_eq!(Solution::calculate(s), res);
}
