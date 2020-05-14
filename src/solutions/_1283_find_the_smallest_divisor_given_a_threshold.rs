struct Solution;

impl Solution {
    fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut l = 1;
        let mut r = 1_000_000;
        while l < r {
            let m = l + (r - l) / 2;
            let mut sum = 0;
            for &x in &nums {
                if x % m == 0 {
                    sum += x / m;
                } else {
                    sum += x / m + 1;
                }
            }
            if sum > threshold {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 5, 9];
    let threshold = 6;
    let res = 5;
    assert_eq!(Solution::smallest_divisor(nums, threshold), res);
    let nums = vec![2, 3, 5, 7, 11];
    let threshold = 11;
    let res = 3;
    assert_eq!(Solution::smallest_divisor(nums, threshold), res);
    let nums = vec![19];
    let threshold = 5;
    let res = 4;
    assert_eq!(Solution::smallest_divisor(nums, threshold), res);
}
