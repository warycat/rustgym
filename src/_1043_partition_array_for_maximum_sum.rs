struct Solution;

impl Solution {
    fn max_sum_after_partitioning(a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let mut dp = vec![0; n + 1];
        let k = k as usize;
        for r in 1..=n {
            let l = r - 1;
            let mut max = a[l];
            for i in 0..k.min(r) {
                max = max.max(a[l - i]);
                dp[r] = dp[r].max(dp[l - i] + max * (i + 1) as i32);
            }
        }
        dp[n]
    }
}

#[test]
fn test() {
    let a = vec![1, 15, 7, 9, 2, 5, 10];
    let k = 3;
    let res = 84;
    assert_eq!(Solution::max_sum_after_partitioning(a, k), res);
}
