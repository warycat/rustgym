struct Solution;

use std::collections::HashMap;

impl Solution {
    fn largest_sum_of_averages(a: Vec<i32>, k: i32) -> f64 {
        let n = a.len();
        let k = k as usize;
        let mut memo: HashMap<(usize, usize), f64> = HashMap::new();
        Self::dp(n, k, &mut memo, &a)
    }
    fn dp(n: usize, k: usize, memo: &mut HashMap<(usize, usize), f64>, a: &[i32]) -> f64 {
        if n == 0 {
            0.0
        } else {
            if let Some(&res) = memo.get(&(n, k)) {
                return res;
            }
            let res = if k == 1 {
                a[0..n].iter().sum::<i32>() as f64 / n as f64
            } else {
                let mut last = 0.0;
                let mut res: f64 = 0.0;
                for i in (0..n).rev() {
                    last += a[i] as f64;
                    let avg = last as f64 / (n - i) as f64;
                    res = res.max(avg + Self::dp(i, k - 1, memo, a));
                }
                res
            };
            memo.insert((n, k), res);
            res
        }
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let a = vec![9, 1, 2, 3, 9];
    let k = 3;
    let res = 20.0;
    assert_approx_eq!(Solution::largest_sum_of_averages(a, k), res);
}
