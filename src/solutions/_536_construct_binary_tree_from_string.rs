struct Solution;
use std::iter::Peekable;
use std::vec::IntoIter;
use util::*;

enum Tok {
    Num(i32),
    Op(char),
}

use Tok::*;

struct TreeParser {
    it: Peekable<IntoIter<Tok>>,
}

impl TreeParser {
    fn new(s: String) -> Self {
        let mut tokens = vec![];
        let mut prev = None;
        for c in s.chars() {
            match c {
                '-' => {
                    tokens.push(Op(c));
                }
                '(' | ')' => {
                    if let Some(val) = prev {
                        tokens.push(Num(val));
                        prev = None;
                    }
                    tokens.push(Op(c));
                }
                _ => {
                    if let Some(mut val) = prev {
                        val *= 10;
                        val += (c as u8 - b'0') as i32;
                        prev = Some(val);
                    } else {
                        prev = Some((c as u8 - b'0') as i32);
                    }
                }
            }
        }
        if let Some(val) = prev {
            tokens.push(Num(val));
        }
        let it = tokens.into_iter().peekable();
        TreeParser { it }
    }

    fn parse(&mut self) -> TreeLink {
        let sign = if self.eat('-') { -1 } else { 1 };
        if let Some(Num(val)) = self.it.next() {
            let mut left = None;
            let mut right = None;
            if self.eat('(') {
                left = self.parse();
                self.eat(')');
            }
            if self.eat('(') {
                right = self.parse();
                self.eat(')');
            }
            tree!(val * sign, left, right)
        } else {
            None
        }
    }

    fn eat(&mut self, c: char) -> bool {
        if let Some(&Op(t)) = self.it.peek() {
            if t == c {
                self.it.next();
                return true;
            }
        }
        false
    }
}

impl Solution {
    fn str2tree(s: String) -> TreeLink {
        TreeParser::new(s).parse()
    }
}

#[test]
fn test() {
    let s = "4(2(3)(1))(6(5))".to_string();
    let res = tree!(4, tree!(2, tree!(3), tree!(1)), tree!(6, tree!(5), None));
    assert_eq!(Solution::str2tree(s), res);
    let s = "-4(2(3)(1))(6(5))".to_string();
    let res = tree!(-4, tree!(2, tree!(3), tree!(1)), tree!(6, tree!(5), None));
    assert_eq!(Solution::str2tree(s), res);
}
