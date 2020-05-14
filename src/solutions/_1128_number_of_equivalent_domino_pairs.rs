struct Solution;

use std::collections::HashMap;

impl Solution {
    fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut hs: HashMap<Vec<i32>, i32> = HashMap::new();
        let mut sum = 0;
        for d in dominoes {
            let a = d[0];
            let b = d[1];
            if a < b {
                *hs.entry(vec![a, b]).or_default() += 1;
            } else {
                *hs.entry(vec![b, a]).or_default() += 1;
            }
        }
        for v in hs.values() {
            sum += v * (v - 1) / 2;
        }
        sum
    }
}

#[test]
fn test() {
    let dominoes: Vec<Vec<i32>> = vec_vec_i32![[1, 2], [2, 1], [3, 4], [5, 6]];
    assert_eq!(Solution::num_equiv_domino_pairs(dominoes), 1);
}
