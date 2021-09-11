use rustgym_util::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let it = reader.lines().map(|l| l.unwrap());
    let mut id: HashMap<String, usize> = HashMap::new();
    let mut adj: HashMap<usize, Vec<(usize, i32)>> = HashMap::new();
    for line in it {
        let words: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        let size = id.len();
        let u = *id.entry(words[0].clone()).or_insert(size);
        let size = id.len();
        let v = *id.entry(words[2].clone()).or_insert(size);
        let val = words[4].parse::<i32>().unwrap();
        adj.entry(u).or_default().push((v, val));
        adj.entry(v).or_default().push((u, val));
    }
    let n = id.len();
    let mut res1 = std::i32::MAX;
    let mut res2 = 0;
    for i in 0..n {
        let mut visited = HashSet::new();
        visited.insert(i);
        dfs(i, 0, &mut visited, &mut res1, &mut res2, &adj, n);
        visited.remove(&i);
    }
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

fn dfs(
    u: usize,
    dist: i32,
    visited: &mut HashSet<usize>,
    min: &mut i32,
    max: &mut i32,
    adj: &HashMap<usize, Vec<(usize, i32)>>,
    n: usize,
) {
    if visited.len() == n {
        *min = (*min).min(dist);
        *max = (*max).max(dist);
    }
    for &(v, d) in &adj[&u] {
        if visited.insert(v) {
            dfs(v, dist + d, visited, min, max, adj, n);
            visited.remove(&v);
        }
    }
}

adventofcode!(test, "input.txt", "output.txt");
