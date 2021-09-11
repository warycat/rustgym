use rustgym_util::*;
use std::fmt::Write;
use std::io::*;
use std::iter::Peekable;
use std::slice::Iter;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let it = reader.lines().map(|l| l.unwrap());
    let mut exprs: Vec<Vec<Tok>> = vec![];
    for line in it {
        let tokens = parse_expr(&line);
        exprs.push(tokens);
    }
    let res1: i64 = exprs.iter().map(|e| eval_expr1(e)).sum();
    let res2: i64 = exprs.iter().map(|e| eval_expr2(e)).sum();
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

#[derive(Debug)]
enum Tok {
    Num(i64),
    Op(char),
}

fn parse_expr(s: &str) -> Vec<Tok> {
    let mut tokens: Vec<Tok> = vec![];
    for c in s.chars() {
        match c {
            '(' | ')' | '+' | '*' => tokens.push(Tok::Op(c)),
            '0'..='9' => tokens.push(Tok::Num((c as u8 - b'0') as i64)),
            _ => {}
        }
    }
    tokens
}

fn eval_expr1(e: &[Tok]) -> i64 {
    let mut it = e.iter().peekable();
    expr1(&mut it)
}

fn eval_expr2(e: &[Tok]) -> i64 {
    let mut it = e.iter().peekable();
    expr2(&mut it)
}

fn expr1(it: &mut Peekable<Iter<Tok>>) -> i64 {
    let mut res = term1(it);
    while let Some(Tok::Op(c)) = it.peek() {
        match c {
            '+' => {
                it.next().unwrap();
                res += term1(it);
            }
            '*' => {
                it.next().unwrap();
                res *= term1(it);
            }
            ')' => {
                break;
            }
            _ => {
                panic!();
            }
        }
    }
    res
}

fn expr2(it: &mut Peekable<Iter<Tok>>) -> i64 {
    let mut res = fact2(it);
    while let Some(Tok::Op(c)) = it.peek() {
        match c {
            '*' => {
                it.next().unwrap();
                res *= fact2(it);
            }
            ')' | '+' => {
                break;
            }
            _ => {
                panic!();
            }
        }
    }
    res
}

fn fact2(it: &mut Peekable<Iter<Tok>>) -> i64 {
    let mut res = term2(it);
    while let Some(Tok::Op(c)) = it.peek() {
        match c {
            '+' => {
                it.next().unwrap();
                res += term2(it);
            }
            ')' | '*' => {
                break;
            }
            _ => {
                panic!();
            }
        }
    }
    res
}

fn term1(it: &mut Peekable<Iter<Tok>>) -> i64 {
    match it.next() {
        Some(&Tok::Num(x)) => x,
        Some(Tok::Op('(')) => {
            let res = expr1(it);
            it.next().unwrap();
            res
        }
        _ => panic!(),
    }
}

fn term2(it: &mut Peekable<Iter<Tok>>) -> i64 {
    match it.next() {
        Some(&Tok::Num(x)) => x,
        Some(Tok::Op('(')) => {
            let res = expr2(it);
            it.next().unwrap();
            res
        }
        _ => panic!(),
    }
}

adventofcode!(test, "input.txt", "output.txt");
