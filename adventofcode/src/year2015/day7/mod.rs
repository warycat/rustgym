use rustgym_util::*;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Write;
use std::io::*;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
enum Tok {
    Id(String),
    Val(u16),
    Op(char),
}

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let it = reader.lines().map(|l| l.unwrap());
    let mut stmt: HashMap<Tok, Vec<Tok>> = HashMap::new();
    let mut adj: HashMap<Tok, Vec<Tok>> = HashMap::new();
    let mut values1: HashMap<Tok, u16> = HashMap::new();
    let mut values2: HashMap<Tok, u16> = HashMap::new();
    for line in it {
        let mut tokens: Vec<Tok> = line.split_whitespace().map(parse_token).collect();
        let last = tokens.pop().unwrap();
        tokens.pop();
        stmt.insert(last.clone(), tokens.to_vec());
        for i in 0..tokens.len() {
            if let Tok::Id(_) = tokens[i] {
                adj.entry(tokens[i].clone()).or_default().push(last.clone());
            }
        }
    }
    let mut ovalues = HashMap::new();
    eval(&mut values1, &adj, &stmt, &ovalues);
    let res1 = *values1.entry(Tok::Id("a".to_string())).or_default();
    ovalues.insert(Tok::Id("b".to_string()), res1);
    eval(&mut values2, &adj, &stmt, &ovalues);
    let res2 = *values2.entry(Tok::Id("a".to_string())).or_default();
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

fn eval(
    values: &mut HashMap<Tok, u16>,
    adj: &HashMap<Tok, Vec<Tok>>,
    stmt: &HashMap<Tok, Vec<Tok>>,
    o: &HashMap<Tok, u16>,
) {
    let mut indegree: HashMap<Tok, usize> = HashMap::new();
    let mut queue: VecDeque<Tok> = VecDeque::new();
    for (u, vs) in adj {
        indegree.entry(u.clone()).or_default();
        for v in vs {
            *indegree.entry(v.clone()).or_default() += 1;
        }
    }
    for (tok, val) in indegree.iter() {
        if *val == 0 {
            queue.push_back(tok.clone());
        }
    }
    while let Some(tok) = queue.pop_front() {
        *values.entry(tok.clone()).or_default() = eval_stmt(&stmt[&tok], values, o);
        for v in adj.get(&tok).unwrap_or(&vec![]) {
            *indegree.entry(v.clone()).or_default() -= 1;
            if indegree[v] == 0 {
                queue.push_back(v.clone());
            }
        }
    }
}

fn eval_stmt(stmt: &[Tok], values: &HashMap<Tok, u16>, o: &HashMap<Tok, u16>) -> u16 {
    match stmt.len() {
        1 => eval_tok(&stmt[0], values, o),
        2 => !eval_tok(&stmt[1], values, o),
        3 => match stmt[1] {
            Tok::Op('&') => eval_tok(&stmt[0], values, o) & eval_tok(&stmt[2], values, o),
            Tok::Op('|') => eval_tok(&stmt[0], values, o) | eval_tok(&stmt[2], values, o),
            Tok::Op('<') => eval_tok(&stmt[0], values, o) << eval_tok(&stmt[2], values, o),
            Tok::Op('>') => eval_tok(&stmt[0], values, o) >> eval_tok(&stmt[2], values, o),
            _ => panic!(),
        },
        _ => panic!(),
    }
}

fn eval_tok(tok: &Tok, values: &HashMap<Tok, u16>, o: &HashMap<Tok, u16>) -> u16 {
    match tok.clone() {
        Tok::Val(val) => val,
        Tok::Id(_) => {
            if let Some(&val) = o.get(tok) {
                val
            } else {
                values[tok]
            }
        }
        _ => panic!(),
    }
}

fn parse_token(s: &str) -> Tok {
    if let Ok(val) = s.parse::<u16>() {
        Tok::Val(val)
    } else {
        match s {
            "AND" => Tok::Op('&'),
            "OR" => Tok::Op('|'),
            "LSHIFT" => Tok::Op('<'),
            "RSHIFT" => Tok::Op('>'),
            "NOT" => Tok::Op('!'),
            "->" => Tok::Op('='),
            _ => Tok::Id(s.to_string()),
        }
    }
}

adventofcode!(test, "input.txt", "output.txt");
