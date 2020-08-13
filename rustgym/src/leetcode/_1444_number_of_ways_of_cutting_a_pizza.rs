struct Solution;

use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let n = pizza.len();
        let m = pizza[0].len();
        let pizza: Vec<Vec<i32>> = pizza
            .into_iter()
            .map(|s| s.chars().map(|c| if c == 'A' { 1 } else { 0 }).collect())
            .collect();
        let mut prefix = vec![vec![0; m]; n];
        for i in (0..n).rev() {
            for j in (0..m).rev() {
                prefix[i][j] = pizza[i][j];
                if i + 1 < n {
                    prefix[i][j] += prefix[i + 1][j];
                }
                if j + 1 < m {
                    prefix[i][j] += prefix[i][j + 1];
                }
                if i + 1 < n && j + 1 < m {
                    prefix[i][j] -= prefix[i + 1][j + 1];
                }
            }
        }
        let mut memo: HashMap<(usize, usize, i32), i64> = HashMap::new();
        (Self::dp(0, 0, k, &mut memo, &prefix, n, m) % MOD) as i32
    }

    fn dp(
        r: usize,
        c: usize,
        k: i32,
        memo: &mut HashMap<(usize, usize, i32), i64>,
        prefix: &[Vec<i32>],
        n: usize,
        m: usize,
    ) -> i64 {
        if let Some(&res) = memo.get(&(r, c, k)) {
            return res;
        }
        let res = if k == 1 {
            if prefix[r][c] > 0 {
                1
            } else {
                0
            }
        } else {
            let mut res = 0;
            for i in r + 1..n {
                let down = prefix[i][c];
                let up = prefix[r][c] - down;
                if up > 0 {
                    res += Self::dp(i, c, k - 1, memo, prefix, n, m);
                }
            }
            for j in c + 1..m {
                let right = prefix[r][j];
                let left = prefix[r][c] - right;
                if left > 0 {
                    res += Self::dp(r, j, k - 1, memo, prefix, n, m);
                }
            }
            res
        };
        memo.insert((r, c, k), res);
        res
    }
}

#[test]
fn test() {
    let pizza = vec_string!["A..", "AAA", "..."];
    let k = 3;
    let res = 3;
    assert_eq!(Solution::ways(pizza, k), res);
    let pizza = vec_string!["A..", "AA.", "..."];
    let k = 3;
    let res = 1;
    assert_eq!(Solution::ways(pizza, k), res);
    let pizza = vec_string!["A..", "A..", "..."];
    let k = 1;
    let res = 1;
    assert_eq!(Solution::ways(pizza, k), res);
}
