struct Solution;

impl Solution {
    fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut cur = vec![];
        let mut res = vec![];
        let n = candidates.len();
        candidates.sort_unstable();
        Self::dfs(0, target, &mut cur, &mut res, &candidates, n);
        res
    }
    fn dfs(
        start: usize,
        target: i32,
        cur: &mut Vec<i32>,
        all: &mut Vec<Vec<i32>>,
        candidates: &[i32],
        n: usize,
    ) {
        use std::cmp::Ordering::*;
        match target.cmp(&0) {
            Equal => {
                all.push(cur.to_vec());
            }
            Greater => {
                for i in start..n {
                    if i > start && candidates[i] == candidates[i - 1] {
                        continue;
                    }
                    cur.push(candidates[i]);
                    Self::dfs(i + 1, target - candidates[i], cur, all, candidates, n);
                    cur.pop();
                }
            }
            Less => {}
        }
    }
}

#[test]
fn test() {
    let candidates = vec![10, 1, 2, 7, 6, 1, 5];
    let target = 8;
    let mut res = vec_vec_i32![[1, 7], [1, 2, 5], [2, 6], [1, 1, 6]];
    res.sort_unstable();
    assert_eq!(Solution::combination_sum2(candidates, target), res);
}
