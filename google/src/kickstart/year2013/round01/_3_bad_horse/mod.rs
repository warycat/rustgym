use rustgym_util::*;
use std::collections::HashMap;
use std::fmt::Write;
use std::io::*;

fn solve(case_no: usize, reader: &mut impl BufRead, writer: &mut impl Write) {
    let m = reader.parse_line();
    let mut ids: HashMap<String, usize> = HashMap::new();
    let pairs: Vec<Vec<usize>> = reader
        .collect_mat(m)
        .iter()
        .map(|row| {
            row.iter()
                .map(|s| {
                    let size = ids.len();
                    *ids.entry(s.to_string()).or_insert(size)
                })
                .collect()
        })
        .collect();
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
    writeln!(writer, "Case #{}: {}", case_no, res).unwrap();
}

google_test_gen!(test, "input.txt", "output.txt");
