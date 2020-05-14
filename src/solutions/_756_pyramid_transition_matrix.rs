struct Solution;

use std::collections::HashSet;

impl Solution {
    fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let n = bottom.len();
        let mut v: Vec<Vec<usize>> = vec![vec![]; n];
        let mut map: Vec<Vec<HashSet<usize>>> = vec![vec![HashSet::new(); 7]; 7];
        for c in bottom.bytes() {
            let b = (c - b'A') as usize;
            v[n - 1].push(b);
        }
        for s in allowed {
            let s: Vec<u8> = s.bytes().collect();
            let a: usize = (s[0] - b'A') as usize;
            let b: usize = (s[1] - b'A') as usize;
            let c: usize = (s[2] - b'A') as usize;
            map[a][b].insert(c);
        }
        Self::backtrack(&mut v, &map, n - 1, n - 1)
    }

    fn backtrack(
        v: &mut Vec<Vec<usize>>,
        map: &[Vec<HashSet<usize>>],
        row: usize,
        col: usize,
    ) -> bool {
        if row == 0 {
            return true;
        }
        let (r, c) = if col == row {
            (row - 1, 0)
        } else {
            (row, col + 1)
        };
        let left = v[r + 1][c];
        let right = v[r + 1][c + 1];
        for &x in &map[left][right] {
            v[r].push(x);
            if Self::backtrack(v, map, r, c) {
                return true;
            }
            v[r].pop();
        }
        false
    }
}

#[test]
fn test() {
    let bottom = "ABC".to_string();
    let allowed: Vec<String> = vec_string!["ABD", "BCE", "DEF", "FFF"];
    Solution::pyramid_transition(bottom, allowed);
}
