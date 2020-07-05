struct Solution;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::iter::Peekable;
use std::str::Chars;
use std::vec::IntoIter;

#[derive(Debug)]
enum Tok {
    S(String),
    Op(char),
}

impl Solution {
    fn brace_expansion_ii(expression: String) -> Vec<String> {
        let mut it = expression.chars().peekable();
        let toks = Self::parse_tokens(&mut it);
        let mut it = toks.into_iter().peekable();
        let mut sorted: Vec<String> = Self::parse(&mut it).into_iter().collect();
        sorted.sort_unstable();
        sorted
    }

    fn concat(a: &HashSet<String>, b: &HashSet<String>) -> HashSet<String> {
        let mut res = HashSet::new();
        for i in a {
            for j in b {
                res.insert(format!("{}{}", i, j));
            }
        }
        res
    }

    fn parse(it: &mut Peekable<IntoIter<Tok>>) -> HashSet<String> {
        let mut res = HashSet::from_iter(vec!["".to_string()]);
        while it.len() != 0 {
            if Self::eat(it, '{') {
                res = Self::concat(&res, &Self::parse(it));
                Self::eat(it, '}');
                continue;
            }
            if Self::eat(it, ',') {
                res = &res | &Self::parse(it);
                continue;
            }
            if let Some(Tok::Op('}')) = it.peek() {
                break;
            }
            if let Some(Tok::S(s)) = it.next() {
                res = Self::concat(&res, &HashSet::from_iter(vec![s]));
                continue;
            }
        }
        res
    }

    fn eat(it: &mut Peekable<IntoIter<Tok>>, c: char) -> bool {
        if let Some(&Tok::Op(t)) = it.peek() {
            if t == c {
                it.next();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn parse_tokens(it: &mut Peekable<Chars>) -> Vec<Tok> {
        let mut res = vec![];
        while let Some(c) = it.next() {
            match c {
                '{' | '}' | ',' => res.push(Tok::Op(c)),
                'a'..='z' => {
                    let mut s = "".to_string();
                    s.push(c);
                    while let Some('a'..='z') = it.peek() {
                        s.push(it.next().unwrap());
                    }
                    res.push(Tok::S(s));
                }
                _ => {}
            }
        }
        res
    }
}

#[test]
fn test() {
    let expression = "{a,b}{c,{d,e}}".to_string();
    let res = vec_string!["ac", "ad", "ae", "bc", "bd", "be"];
    assert_eq!(Solution::brace_expansion_ii(expression), res);
    let expression = "{{a,z},a{b,c},{ab,z}}".to_string();
    let res = vec_string!["a", "ab", "ac", "z"];
    assert_eq!(Solution::brace_expansion_ii(expression), res);
}
