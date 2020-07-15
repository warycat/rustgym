struct Solution;

use std::collections::HashMap;

impl Solution {
    fn min_transfers(transactions: Vec<Vec<i32>>) -> i32 {
        let mut balance: HashMap<i32, i32> = HashMap::new();
        for t in transactions {
            *balance.entry(t[0]).or_default() -= t[2];
            *balance.entry(t[1]).or_default() += t[2];
        }
        let mut debts = vec![];
        for &v in balance.values() {
            if v != 0 {
                debts.push(v);
            }
        }
        let n = debts.len();
        let mut res = std::usize::MAX;
        Self::dfs(0, 0, &mut debts, &mut res, n);
        res as i32
    }

    fn dfs(start: usize, cur: usize, debts: &mut Vec<i32>, min: &mut usize, n: usize) {
        if start == n {
            *min = cur.min(*min);
        } else {
            if debts[start] == 0 {
                Self::dfs(start + 1, cur, debts, min, n);
            } else {
                let transaction = debts[start];
                debts[start] -= transaction;
                for i in start + 1..n {
                    if debts[i] * transaction < 0 {
                        debts[i] += transaction;
                        Self::dfs(start + 1, cur + 1, debts, min, n);
                        debts[i] -= transaction;
                    }
                }
                debts[start] += transaction;
            }
        }
    }
}

#[test]
fn test() {
    let transactions = vec_vec_i32![[0, 1, 10], [2, 0, 5]];
    let res = 2;
    assert_eq!(Solution::min_transfers(transactions), res);
    let transactions = vec_vec_i32![[0, 1, 10], [1, 0, 1], [1, 2, 5], [2, 0, 5]];
    let res = 1;
    assert_eq!(Solution::min_transfers(transactions), res);
}
