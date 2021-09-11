use rustgym_util::*;
use std::collections::HashMap;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let it = reader.lines().map(|l| l.unwrap());
    let mut color: HashMap<(String, String), usize> = HashMap::new();
    let mut rules: Vec<Vec<String>> = vec![];
    for line in it {
        let words: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        rules.push(words);
    }
    for rule in &rules {
        let size = color.len();
        color
            .entry((rule[0].to_string(), rule[1].to_string()))
            .or_insert(size);
    }
    let k = color[&("shiny".to_string(), "gold".to_string())];
    let n = color.len();
    let mut adj: Vec<Vec<(usize, i32)>> = vec![vec![]; n];

    for rule in &rules {
        if rule[4] == "no" {
            continue;
        }
        let mut i = 4;
        let u = color[&(rule[0].clone(), rule[1].clone())];
        while i < rule.len() {
            let d = rule[i].parse::<i32>().unwrap();
            let v = color[&(rule[i + 1].clone(), rule[i + 2].clone())];
            adj[u].push((v, d));
            i += 4;
        }
    }
    let mut visited: Vec<bool> = vec![false; n];
    let mut res1 = 0;
    for i in 0..n {
        if i == k {
            continue;
        }
        if dfs1(i, &mut visited, &adj, n, k) {
            res1 += 1;
        }
    }
    let res2 = dfs2(k, &mut visited, &adj, n) - 1;
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

fn dfs1(u: usize, visited: &mut Vec<bool>, adj: &[Vec<(usize, i32)>], n: usize, k: usize) -> bool {
    if u == k {
        true
    } else {
        let mut res = false;
        visited[u] = true;
        for &(v, _) in &adj[u] {
            if dfs1(v, visited, adj, n, k) {
                res = true;
                break;
            }
        }
        visited[u] = false;
        res
    }
}

fn dfs2(u: usize, visited: &mut Vec<bool>, adj: &[Vec<(usize, i32)>], n: usize) -> i32 {
    let mut res = 1;
    visited[u] = true;
    if !adj[u].is_empty() {
        for &(v, d) in &adj[u] {
            res += d * dfs2(v, visited, adj, n);
        }
    }
    visited[u] = false;
    res
}

adventofcode!(test, "input.txt", "output.txt");
