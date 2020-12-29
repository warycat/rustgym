struct Solution;

use std::collections::HashMap;

impl Solution {
    fn split_array_same_average(a: Vec<i32>) -> bool {
        let n = a.len();
        let sum: i32 = a.iter().sum();
        for k in 1..=n / 2 {
            if sum * k as i32 % n as i32 == 0 {
                let target = sum * k as i32 / n as i32;
                if Self::dp(target, k, n, &mut HashMap::new(), &a) {
                    return true;
                }
            }
        }
        false
    }
    fn dp(
        target: i32,
        k: usize,
        n: usize,
        memo: &mut HashMap<(i32, usize, usize), bool>,
        a: &[i32],
    ) -> bool {
        if let Some(&res) = memo.get(&(target, k, n)) {
            res
        } else {
            let res = if k == 0 {
                target == 0
            } else {
                if n < k {
                    false
                } else {
                    let next = target - a[n - 1];
                    Self::dp(target, k, n - 1, memo, a)
                        || next >= 0 && Self::dp(next, k - 1, n - 1, memo, a)
                }
            };
            memo.insert((target, k, n), res);
            res
        }
    }
}

#[test]
fn test() {
    let a = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let res = true;
    assert_eq!(Solution::split_array_same_average(a), res);
}
