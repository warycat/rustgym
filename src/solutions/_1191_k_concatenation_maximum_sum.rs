struct Solution;

const MOD: i32 = 1_000_000_007;

impl Solution {
    fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
        let sum: i32 = arr.iter().sum();
        let mut prev = 0;
        let mut res = 0;
        let mut k = k as usize;
        let n = arr.len();
        for i in 0..n * k.min(2) {
            prev = arr[i % n].max(prev + arr[i % n]);
            res = res.max(prev);
        }
        while sum > 0 && k > 2 {
            res += sum;
            res %= MOD;
            k -= 1
        }
        res
    }
}

#[test]
fn test() {
    let arr = vec![1, 2];
    let k = 3;
    let res = 9;
    assert_eq!(Solution::k_concatenation_max_sum(arr, k), res);
    let arr = vec![1, -2, 1];
    let k = 5;
    let res = 2;
    assert_eq!(Solution::k_concatenation_max_sum(arr, k), res);
    let arr = vec![-1, -2];
    let k = 7;
    let res = 0;
    assert_eq!(Solution::k_concatenation_max_sum(arr, k), res);
}
