struct Solution;
use std::collections::BTreeMap;

impl Solution {
    fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let n = difficulty.len();
        let mut btm: BTreeMap<i32, i32> = BTreeMap::new();
        for i in 0..n {
            let v = btm.entry(difficulty[i]).or_default();
            *v = profit[i].max(*v);
        }
        let mut prev = 0;
        for (_, v) in btm.iter_mut() {
            if prev > *v {
                *v = prev;
            }
            prev = *v;
        }
        let mut res = 0;
        for w in worker {
            res += *btm.range(0..=w).rev().map(|(_, v)| v).next().unwrap_or(&0);
        }
        res
    }
}

#[test]
fn test() {
    let difficulty = vec![2, 4, 6, 8, 10];
    let profit = vec![10, 20, 30, 40, 50];
    let worker = vec![4, 5, 6, 7];
    let res = 100;
    assert_eq!(
        Solution::max_profit_assignment(difficulty, profit, worker),
        res
    );
}
