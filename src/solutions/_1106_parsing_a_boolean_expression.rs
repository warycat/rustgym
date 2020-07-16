struct Solution;

use std::iter::Peekable;
use std::str::Chars;
use std::vec::IntoIter;

enum Tok {
    Bool(bool),
    Op(char),
}

impl Solution {
    fn parse_bool_expr(expression: String) -> bool {
        let tokens = Self::parse_tokens(&mut expression.chars());
        let mut it = tokens.into_iter().peekable();
        Self::parse(&mut it)
    }

    fn parse(it: &mut Peekable<IntoIter<Tok>>) -> bool {
        let tok = it.next().unwrap();
        match tok {
            Tok::Bool(b) => b,
            Tok::Op('!') => {
                it.next();
                let res = !Self::parse(it);
                it.next();
                res
            }
            Tok::Op('&') => {
                it.next();
                let mut res = Self::parse(it);
                while let Some(&Tok::Op(',')) = it.peek() {
                    it.next();
                    res &= Self::parse(it);
                }
                it.next();
                res
            }
            Tok::Op('|') => {
                it.next();
                let mut res = Self::parse(it);
                while let Some(&Tok::Op(',')) = it.peek() {
                    it.next();
                    res |= Self::parse(it);
                }
                it.next();
                res
            }
            _ => panic!(),
        }
    }

    fn parse_tokens(it: &mut Chars) -> Vec<Tok> {
        let mut res = vec![];
        for c in it {
            let tok = match c {
                't' => Tok::Bool(true),
                'f' => Tok::Bool(false),
                _ => Tok::Op(c),
            };
            res.push(tok);
        }
        res
    }
}

#[test]
fn test() {
    let expression = "!(f)".to_string();
    let res = true;
    assert_eq!(Solution::parse_bool_expr(expression), res);
    let expression = "|(f,t)".to_string();
    let res = true;
    assert_eq!(Solution::parse_bool_expr(expression), res);
    let expression = "&(t,f)".to_string();
    let res = false;
    assert_eq!(Solution::parse_bool_expr(expression), res);
    let expression = "|(&(t,f,t),!(t))".to_string();
    let res = false;
    assert_eq!(Solution::parse_bool_expr(expression), res);
}
