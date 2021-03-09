struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut odd_count = 0;
        let mut res = 0;
        let mut sum_is_odd = 0;
        let mut sum_is_even = 1;
        for x in arr {
            if x % 2 == 1 {
                odd_count += 1;
            }
            if odd_count % 2 == 1 {
                res += sum_is_even;
                res %= MOD;
                sum_is_odd += 1;
            } else {
                res += sum_is_odd;
                res %= MOD;
                sum_is_even += 1;
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let arr = vec![1, 3, 5];
    let res = 4;
    assert_eq!(Solution::num_of_subarrays(arr), res);
    let arr = vec![2, 4, 6];
    let res = 0;
    assert_eq!(Solution::num_of_subarrays(arr), res);
    let arr = vec![1, 2, 3, 4, 5, 6, 7];
    let res = 16;
    assert_eq!(Solution::num_of_subarrays(arr), res);
    let arr = vec![100, 100, 99, 99];
    let res = 4;
    assert_eq!(Solution::num_of_subarrays(arr), res);
    let arr = vec![7];
    let res = 1;
    assert_eq!(Solution::num_of_subarrays(arr), res);
}
