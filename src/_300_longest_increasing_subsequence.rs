struct Solution;

impl Solution {
    fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = vec![];
        for x in nums {
            if let Err(i) = dp.binary_search(&x) {
                if i == dp.len() {
                    dp.push(x)
                } else {
                    dp[i] = x;
                }
            }
        }
        dp.len() as i32
    }
}

#[test]
fn test() {
    let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
    let res = 4;
    assert_eq!(Solution::length_of_lis(nums), res);
}
