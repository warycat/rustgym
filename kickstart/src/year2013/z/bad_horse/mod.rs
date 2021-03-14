use rustgym_util::*;
use std::collections::HashMap;
use std::fmt::Write;
use std::io::*;

fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) -> RustGymResult {
    let mut it = reader.lines().map(|l| l.unwrap());
    let t = it.next().unwrap().parse::<usize>()?;
    for i in 1..=t {
        let m = it.next().unwrap().parse::<usize>()?;
        let mut ids: HashMap<String, usize> = HashMap::new();
        let mut pairs: Vec<Vec<usize>> = vec![];
        for _ in 0..m {
            let names: Vec<usize> = it
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| {
                    let size = ids.len();
                    *ids.entry(s.to_string()).or_insert(size)
                })
                .collect();
            pairs.push(names);
        }
        let n = ids.len();
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        for pair in pairs {
            let u = pair[0];
            let v = pair[1];
            adj[u].push(v);
            adj[v].push(u);
        }
        let mut uf = UnionFind::new(n);
        for u in 0..n {
            let size = adj[u].len();
            for i in 1..size {
                uf.union(adj[u][0], adj[u][i]);
            }
        }
        let res = if uf.group() == 1 {
            "No".to_string()
        } else {
            "Yes".to_string()
        };
        writeln!(writer, "Case #{}: {}", i, res)?;
    }
    Ok(())
}

test_gen!(test, "input.txt", "output.txt");
