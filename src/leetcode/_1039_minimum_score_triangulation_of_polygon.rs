struct Solution;
use std::collections::HashMap;

impl Solution {
    fn min_score_triangulation(a: Vec<i32>) -> i32 {
        let n = a.len();
        let mut memo: HashMap<(usize, usize), i32> = HashMap::new();
        Self::dp(0, n - 1, &mut memo, &a, n)
    }

    fn dp(i: usize, j: usize, memo: &mut HashMap<(usize, usize), i32>, a: &[i32], n: usize) -> i32 {
        if let Some(&res) = memo.get(&(i, j)) {
            return res;
        }
        let mut res = std::i32::MAX;
        for k in i + 1..j {
            let left = Self::dp(i, k, memo, a, n);
            let right = Self::dp(k, j, memo, a, n);
            let mid = a[i] * a[j] * a[k];
            res = res.min(left + right + mid);
        }
        if res == std::i32::MAX {
            res = 0;
        }
        memo.insert((i, j), res);
        res
    }
}

#[test]
fn test() {
    let a = vec![1, 2, 3];
    let res = 6;
    assert_eq!(Solution::min_score_triangulation(a), res);
    let a = vec![3, 7, 4, 5];
    let res = 144;
    assert_eq!(Solution::min_score_triangulation(a), res);
    let a = vec![1, 3, 1, 4, 1, 5];
    let res = 13;
    assert_eq!(Solution::min_score_triangulation(a), res);
}
