#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

#[macro_export]
macro_rules! nested_integer {
    ($string:expr) => {{
        if let Some(res) = NestedIntegerParser::new($string).parse() {
            res
        } else {
            panic!()
        }
    }};
}

use std::iter::Peekable;
use std::vec::IntoIter;
use NestedInteger::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tok {
    Num(i32),
    Op(char),
}

use Tok::*;

pub struct NestedIntegerParser {
    it: Peekable<IntoIter<Tok>>,
}

impl NestedIntegerParser {
    pub fn new<T: Into<String>>(s: T) -> Self {
        let s: String = s.into();
        let mut tokens: Vec<Tok> = vec![];
        let mut it = s.chars().peekable();
        while let Some(c) = it.next() {
            match c {
                '0'..='9' => {
                    let mut val = (c as u8 - b'0') as i32;
                    while let Some(&c) = it.peek() {
                        match c {
                            '0'..='9' => {
                                it.next();
                                val *= 10;
                                val += (c as u8 - b'0') as i32;
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    tokens.push(Num(val));
                }
                _ => {
                    tokens.push(Op(c));
                }
            }
        }
        NestedIntegerParser {
            it: tokens.into_iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Option<NestedInteger> {
        if !self.eat('[') {
            return self.parse_int();
        }
        let mut list: Vec<NestedInteger> = vec![];
        while let Some(x) = self.parse() {
            list.push(x);
            self.eat(',');
        }
        if !self.eat(']') {
            return None;
        }
        Some(List(list))
    }

    fn parse_int(&mut self) -> Option<NestedInteger> {
        let sign = if self.eat('-') { -1 } else { 1 };
        if let Some(&Tok::Num(num)) = self.it.peek() {
            self.it.next();
            return Some(Int(sign * num));
        }
        None
    }

    fn eat(&mut self, c: char) -> bool {
        if let Some(&Tok::Op(t)) = self.it.peek() {
            if t == c {
                self.it.next();
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
