use rustgym_util::*;
use std::collections::HashMap;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let mut it = reader.lines().map(|l| l.unwrap());
    let mut rules: HashMap<i32, Rule> = HashMap::new();
    let mut messages: Vec<Vec<char>> = vec![];
    while let Some(line) = it.next() {
        if line.is_empty() {
            break;
        }
        let parts: Vec<String> = line.split(':').map(|s| s.to_string()).collect();
        let lhs = parts[0].parse::<i32>().unwrap();
        let words: Vec<String> = parts[1].split_whitespace().map(|s| s.to_string()).collect();
        if words[0].parse::<i32>().is_err() {
            let v: Vec<char> = words[0].chars().collect();
            rules.insert(lhs, Rule::Single(v[1]));
        } else {
            let multi: Vec<Vec<i32>> = words
                .split(|x| x == "|")
                .map(|v| v.iter().map(|x| x.parse::<i32>().unwrap()).collect())
                .collect();
            rules.insert(lhs, Rule::Multi(multi));
        }
    }
    while let Some(line) = it.next() {
        let message = line.chars().collect();
        messages.push(message);
    }
    let res1 = messages.iter().filter(|m| is_valid(m, &rules)).count();
    rules.insert(8, Rule::Multi(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule::Multi(vec![vec![42, 31], vec![42, 11, 31]]));
    let res2 = messages.iter().filter(|m| is_valid(m, &rules)).count();
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Rule {
    Single(char),
    List(Vec<i32>),
    Multi(Vec<Vec<i32>>),
}

fn is_valid(message: &[char], rules: &HashMap<i32, Rule>) -> bool {
    let n = message.len();
    dp(
        0,
        n - 1,
        Rule::List(vec![0]),
        &mut HashMap::new(),
        message,
        rules,
    )
}

fn dp(
    left: usize,
    right: usize,
    rule: Rule,
    memo: &mut HashMap<(usize, usize, Rule), bool>,
    message: &[char],
    rules: &HashMap<i32, Rule>,
) -> bool {
    if let Some(&res) = memo.get(&(left, right, rule.clone())) {
        res
    } else {
        let res = match rule.clone() {
            Rule::Single(c) => left == right && message[left] == c,
            Rule::List(mut list) => {
                let last = list.pop().unwrap();
                if list.is_empty() {
                    dp(left, right, rules[&last].clone(), memo, message, rules)
                } else {
                    let mut res = false;
                    for mid in left..right {
                        if dp(left, mid, Rule::List(list.clone()), memo, message, rules)
                            && dp(mid + 1, right, rules[&last].clone(), memo, message, rules)
                        {
                            res = true;
                            break;
                        }
                    }
                    res
                }
            }
            Rule::Multi(multi) => multi
                .iter()
                .any(|list| dp(left, right, Rule::List(list.clone()), memo, message, rules)),
        };
        memo.insert((left, right, rule), res);
        res
    }
}

adventofcode_ignore!(test, "input.txt", "output.txt");
