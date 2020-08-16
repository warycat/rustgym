struct Solution;

use std::collections::BTreeMap;
use std::iter::Peekable;
use std::str::Chars;
use std::vec::IntoIter;

#[derive(Debug)]
enum Tok {
    Num(usize),
    Name(String),
    Op(char),
}

impl Solution {
    fn count_of_atoms(formula: String) -> String {
        let mut it = formula.chars().peekable();
        let tokens = Self::parse_tokens(&mut it);
        let count: BTreeMap<String, usize> = Self::parse(&mut tokens.into_iter().peekable());
        count
            .into_iter()
            .map(|(k, v)| if v > 1 { format!("{}{}", k, v) } else { k })
            .collect()
    }

    fn parse(it: &mut Peekable<IntoIter<Tok>>) -> BTreeMap<String, usize> {
        let mut res: BTreeMap<String, usize> = BTreeMap::new();
        loop {
            match it.peek() {
                Some(Tok::Name(_)) => {
                    if let Some(Tok::Name(s)) = it.next() {
                        let x = if let Some(&Tok::Num(x)) = it.peek() {
                            it.next();
                            x
                        } else {
                            1
                        };
                        *res.entry(s).or_default() += x;
                    }
                }
                Some(&Tok::Op('(')) => {
                    it.next();
                    let inside = Self::parse(it);
                    it.next();
                    let x = if let Some(&Tok::Num(x)) = it.peek() {
                        it.next();
                        x
                    } else {
                        1
                    };
                    for (k, v) in inside {
                        *res.entry(k).or_default() += v * x;
                    }
                }
                Some(&Tok::Op(')')) | None => {
                    break;
                }
                _ => panic!(),
            }
        }
        res
    }

    fn parse_tokens(it: &mut Peekable<Chars>) -> Vec<Tok> {
        let mut res = vec![];
        while let Some(c) = it.next() {
            match c {
                '(' | ')' => res.push(Tok::Op(c)),
                '0'..='9' => {
                    let mut x = (c as u8 - b'0') as usize;
                    while let Some('0'..='9') = it.peek() {
                        x *= 10;
                        let y = (it.next().unwrap() as u8 - b'0') as usize;
                        x += y;
                    }
                    res.push(Tok::Num(x));
                }
                'A'..='Z' => {
                    let mut s = "".to_string();
                    s.push(c);
                    while let Some('a'..='z') = it.peek() {
                        s.push(it.next().unwrap());
                    }
                    res.push(Tok::Name(s));
                }
                _ => panic!(),
            }
        }
        res
    }
}

#[test]
fn test() {
    let formula = "H2O".to_string();
    let res = "H2O".to_string();
    assert_eq!(Solution::count_of_atoms(formula), res);
    let formula = "Mg(OH)2".to_string();
    let res = "H2MgO2".to_string();
    assert_eq!(Solution::count_of_atoms(formula), res);
    let formula = "K4(ON(SO3)2)2".to_string();
    let res = "K4N2O14S4".to_string();
    assert_eq!(Solution::count_of_atoms(formula), res);
}
