struct Solution;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        let mut hs: HashSet<String> = HashSet::new();
        let mut mutation_bank: HashSet<String> = HashSet::new();
        hs.insert(start.clone());
        hs.insert(end.clone());
        for gene in bank {
            hs.insert(gene.clone());
            mutation_bank.insert(gene);
        }
        let n = hs.len();
        let mut hm: HashMap<String, usize> = HashMap::new();
        for (i, gene) in hs.into_iter().enumerate() {
            hm.insert(gene, i);
        }
        let mut edges: Vec<Vec<usize>> = vec![vec![]; n];
        for (gene, &u) in &hm {
            let gene: Vec<char> = gene.chars().collect();
            let n = gene.len();
            let mut mutation = gene.to_vec();
            for i in 0..n {
                for &c in &['A', 'C', 'G', 'T'] {
                    if c != gene[i] {
                        mutation[i] = c;
                        let new_gene: String = mutation.iter().collect();
                        if !mutation_bank.contains(&new_gene) {
                            continue;
                        }
                        if let Some(&v) = &hm.get(&new_gene) {
                            edges[u].push(v);
                        }
                    }
                }
                mutation[i] = gene[i];
            }
        }
        let mut visited: Vec<bool> = vec![false; n];
        let mut queue: VecDeque<(usize, i32)> = VecDeque::new();
        let start_id = hm[&start];
        let end_id = hm[&end];
        queue.push_back((start_id, 0));
        while let Some((u, d)) = queue.pop_front() {
            visited[u] = true;
            if u == end_id {
                return d;
            }
            for &v in &edges[u] {
                if !visited[v] {
                    queue.push_back((v, d + 1));
                }
            }
        }
        -1
    }
}

#[test]
fn test() {
    let start = "AACCGGTT".to_string();
    let end = "AACCGGTA".to_string();
    let bank = vec_string!["AACCGGTA"];
    let res = 1;
    assert_eq!(Solution::min_mutation(start, end, bank), res);
    let start = "AACCGGTT".to_string();
    let end = "AAACGGTA".to_string();
    let bank = vec_string!["AACCGGTA", "AACCGCTA", "AAACGGTA"];
    let res = 2;
    assert_eq!(Solution::min_mutation(start, end, bank), res);
    let start = "AAAAACCC".to_string();
    let end = "AACCCCCC".to_string();
    let bank = vec_string!["AAAACCCC", "AAACCCCC", "AACCCCCC"];
    let res = 3;
    assert_eq!(Solution::min_mutation(start, end, bank), res);
    let start = "AACCGGTT".to_string();
    let end = "AACCGGTA".to_string();
    let bank = vec_string![];
    let res = -1;
    assert_eq!(Solution::min_mutation(start, end, bank), res);
}
