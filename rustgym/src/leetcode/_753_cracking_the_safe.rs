struct Solution;

use std::collections::HashSet;

impl Solution {
    fn crack_safe(n: i32, k: i32) -> String {
        let n = n as usize;
        let k = k as usize;
        let mut seq = vec![0; n];
        let mut visited: HashSet<Vec<u8>> = HashSet::new();
        visited.insert(seq.clone());
        let size = k.pow(n as u32);
        Self::dfs(1, &mut visited, &mut seq, k, n, size);
        seq.into_iter().map(|b| (b'0' + b) as char).collect()
    }
    fn dfs(
        start: usize,
        visited: &mut HashSet<Vec<u8>>,
        seq: &mut Vec<u8>,
        k: usize,
        n: usize,
        size: usize,
    ) -> bool {
        if visited.len() == size {
            return true;
        }
        for i in 0..k {
            let mut suffix = seq[start..].to_vec();
            suffix.push(i as u8);
            if visited.insert(suffix.clone()) {
                seq.push(i as u8);
                if Self::dfs(start + 1, visited, seq, k, n, size) {
                    return true;
                };
                seq.pop();
                visited.remove(&suffix);
            }
        }
        false
    }
}

#[test]
fn test() {
    let n = 1;
    let k = 2;
    let res = "01".to_string();
    assert_eq!(Solution::crack_safe(n, k), res);
    let n = 2;
    let k = 2;
    let res = "00110".to_string();
    assert_eq!(Solution::crack_safe(n, k), res);
}
