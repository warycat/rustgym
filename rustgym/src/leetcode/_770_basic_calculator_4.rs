struct Solution;
use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::iter::Peekable;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;
use std::slice::Iter;
use std::str::Chars;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Term {
    co: i32,
    va: Vec<String>,
}

impl Term {
    fn new(co: i32, va: Vec<String>) -> Self {
        Term { co, va }
    }
}

impl Neg for Term {
    type Output = Term;
    fn neg(self) -> Self::Output {
        Term::new(-self.co, self.va)
    }
}

impl Add for Term {
    type Output = Term;
    fn add(self, rhs: Term) -> Self::Output {
        if self.va != rhs.va {
            panic!();
        }
        Term::new(self.co + rhs.co, self.va)
    }
}

impl Sub for Term {
    type Output = Term;
    fn sub(self, rhs: Term) -> Self::Output {
        if self.va != rhs.va {
            panic!();
        }
        Term::new(self.co - rhs.co, self.va)
    }
}

impl Mul for Term {
    type Output = Term;
    fn mul(self, rhs: Term) -> Self::Output {
        let co = self.co * rhs.co;
        let mut va = vec![];
        let mut left_va = self.va;
        let mut right_va = rhs.va;
        va.append(&mut left_va);
        va.append(&mut right_va);
        va.sort_unstable();
        Term::new(co, va)
    }
}

struct Expr {
    terms: Vec<Term>,
}

impl Expr {
    fn new(terms: Vec<Term>) -> Self {
        Expr { terms }
    }
}

impl Add for Expr {
    type Output = Expr;
    fn add(self, rhs: Expr) -> Self::Output {
        let mut terms = vec![];
        let mut left_terms = self.terms;
        let mut right_terms = rhs.terms;
        terms.append(&mut left_terms);
        terms.append(&mut right_terms);
        Expr::new(terms)
    }
}

impl Sub for Expr {
    type Output = Expr;
    fn sub(self, rhs: Expr) -> Self::Output {
        let mut terms = vec![];
        let mut left_terms = self.terms;
        let mut right_terms = rhs.terms.into_iter().map(|t| -t).collect();
        terms.append(&mut left_terms);
        terms.append(&mut right_terms);
        Expr::new(terms)
    }
}

impl Mul for Expr {
    type Output = Expr;
    fn mul(self, rhs: Expr) -> Self::Output {
        let mut terms = vec![];
        let left_terms = self.terms;
        let right_terms = rhs.terms;
        for i in 0..left_terms.len() {
            for j in 0..right_terms.len() {
                terms.push(left_terms[i].clone() * right_terms[j].clone());
            }
        }
        Expr::new(terms)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Tok {
    Num(i32),
    Op(char),
    Var(String),
}

impl Tok {
    fn eval(self, lhs: Expr, rhs: Expr) -> Option<Expr> {
        match self {
            Op('+') => Some(lhs + rhs),
            Op('-') => Some(lhs - rhs),
            Op('*') => Some(lhs * rhs),
            _ => None,
        }
    }
}

use Tok::*;

impl Solution {
    fn basic_calculator_iv(
        expression: String,
        evalvars: Vec<String>,
        evalints: Vec<i32>,
    ) -> Vec<String> {
        let mut mapping: HashMap<String, i32> = HashMap::new();
        let n = evalints.len();
        for i in 0..n {
            mapping.insert(evalvars[i].to_string(), evalints[i]);
        }
        let mut it = expression.chars().peekable();
        let tokens = Self::parse_tokens(&mut it, mapping);
        let mut it = tokens.iter().peekable();
        let expr = Self::parse_expr(&mut it).unwrap();
        let mut terms = expr.terms;
        terms.sort_by_key(|t| {
            let mut va = t.va.to_vec();
            va.sort_unstable();
            (Reverse(t.va.len()), va)
        });
        let mut groups: BTreeMap<(Reverse<usize>, Vec<String>), i32> = BTreeMap::new();
        for term in terms {
            *groups
                .entry((Reverse(term.va.len()), term.va.to_vec()))
                .or_default() += term.co;
        }
        let mut res = vec![];
        for ((_, va), co) in groups {
            if co == 0 {
                continue;
            }
            let mut s = co.to_string();
            if !va.is_empty() {
                for x in va {
                    s.push('*');
                    s.push_str(&x);
                }
            }
            res.push(s);
        }
        res
    }

    fn parse_tokens(it: &mut Peekable<Chars>, mapping: HashMap<String, i32>) -> Vec<Tok> {
        let mut res: Vec<Tok> = vec![];
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
                    res.push(Tok::Num(x));
                }
                'a'..='z' => {
                    let mut s = "".to_string();
                    s.push(c);
                    while let Some(c) = it.peek() {
                        if c.is_alphabetic() {
                            s.push(it.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    if let Some(&x) = mapping.get(&s) {
                        res.push(Tok::Num(x));
                    } else {
                        res.push(Tok::Var(s));
                    }
                }
                '+' | '-' | '*' | '/' | '(' | ')' => {
                    res.push(Tok::Op(c));
                }
                _ => {}
            }
        }
        res
    }

    fn parse_expr(it: &mut Peekable<Iter<Tok>>) -> Option<Expr> {
        let mut lhs = Self::parse_factor(it)?;
        while let Some(&tok) = it.peek() {
            if let Op('+') | Op('-') = tok {
                let op = it.next().unwrap().clone();
                let rhs = Self::parse_factor(it)?;
                lhs = op.eval(lhs, rhs)?;
            } else {
                break;
            }
        }
        Some(lhs)
    }

    fn parse_factor(it: &mut Peekable<Iter<Tok>>) -> Option<Expr> {
        let mut lhs = Self::parse_terminal(it)?;
        while let Some(&tok) = it.peek() {
            if let Op('*') | Op('/') = tok {
                let op = it.next().unwrap().clone();
                let rhs = Self::parse_terminal(it)?;
                lhs = op.eval(lhs, rhs)?;
            } else {
                break;
            }
        }
        Some(lhs)
    }

    fn parse_terminal(it: &mut Peekable<Iter<Tok>>) -> Option<Expr> {
        if let Some(Op('(')) = it.peek() {
            it.next();
            let expr = Self::parse_expr(it);
            it.next();
            expr
        } else {
            Self::parse_var(it)
        }
    }

    fn parse_var(it: &mut Peekable<Iter<Tok>>) -> Option<Expr> {
        match it.next() {
            Some(&Tok::Num(x)) => Some(Expr::new(vec![Term::new(x, vec![])])),
            Some(Tok::Var(s)) => Some(Expr::new(vec![Term::new(1, vec![s.to_string()])])),
            _ => None,
        }
    }
}

#[test]
fn test() {
    let expression = "e + 8 - a + 5".to_string();
    let evalvars = vec_string!["e"];
    let evalints = vec![1];
    let res = vec_string!["-1*a", "14"];
    assert_eq!(
        Solution::basic_calculator_iv(expression, evalvars, evalints),
        res
    );
    let expression = "e - 8 + temperature - pressure".to_string();
    let evalvars = vec_string!["e", "temperature"];
    let evalints = vec![1, 12];
    let res = vec_string!["-1*pressure", "5"];
    assert_eq!(
        Solution::basic_calculator_iv(expression, evalvars, evalints),
        res
    );
    let expression = "(e + 8) * (e - 8)".to_string();
    let evalvars = vec_string![];
    let evalints = vec![];
    let res = vec_string!["1*e*e", "-64"];
    assert_eq!(
        Solution::basic_calculator_iv(expression, evalvars, evalints),
        res
    );
    let expression = "7 - 7".to_string();
    let evalvars = vec_string![];
    let evalints = vec![];
    let res = vec_string![];
    assert_eq!(
        Solution::basic_calculator_iv(expression, evalvars, evalints),
        res
    );
    let expression = "a * b * c + b * a * c * 4".to_string();
    let evalvars = vec_string![];
    let evalints = vec![];
    let res = vec_string!["5*a*b*c"];
    assert_eq!(
        Solution::basic_calculator_iv(expression, evalvars, evalints),
        res
    );
    let expression = "((a - b) * (b - c) + (c - a)) * ((a - b) + (b - c) * (c - a))".to_string();
    let evalvars = vec_string![];
    let evalints = vec![];
    let res = vec_string![
        "-1*a*a*b*b",
        "2*a*a*b*c",
        "-1*a*a*c*c",
        "1*a*b*b*b",
        "-1*a*b*b*c",
        "-1*a*b*c*c",
        "1*a*c*c*c",
        "-1*b*b*b*c",
        "2*b*b*c*c",
        "-1*b*c*c*c",
        "2*a*a*b",
        "-2*a*a*c",
        "-2*a*b*b",
        "2*a*c*c",
        "1*b*b*b",
        "-1*b*b*c",
        "1*b*c*c",
        "-1*c*c*c",
        "-1*a*a",
        "1*a*b",
        "1*a*c",
        "-1*b*c"
    ];
    assert_eq!(
        Solution::basic_calculator_iv(expression, evalvars, evalints),
        res
    );
}
