struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let mut sums = vec![];
        let n = n as usize;
        for i in 0..n {
            let k = sums.len();
            for j in 0..i {
                sums.push(sums[k - 1 - j] + nums[i]);
            }
            sums.push(nums[i]);
        }
        sums.sort_unstable();
        let mut res = 0;
        let start = left as usize - 1;
        let end = right as usize;
        for i in start..end {
            res += sums[i] as i64;
            res %= MOD;
        }
        res as i32
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 4];
    let n = 4;
    let left = 1;
    let right = 5;
    let res = 13;
    assert_eq!(Solution::range_sum(nums, n, left, right), res);
    let nums = vec![1, 2, 3, 4];
    let n = 4;
    let left = 3;
    let right = 4;
    let res = 6;
    assert_eq!(Solution::range_sum(nums, n, left, right), res);
    let nums = vec![1, 2, 3, 4];
    let n = 4;
    let left = 1;
    let right = 10;
    let res = 50;
    assert_eq!(Solution::range_sum(nums, n, left, right), res);
}
