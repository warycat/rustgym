struct Solution;

impl Solution {
    fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let n = nums.len();
        let limit = limit as usize;
        let mut delta = vec![0; limit * 2 + 2];
        for i in 0..n / 2 {
            let a = nums[i] as usize;
            let b = nums[n - 1 - i] as usize;
            delta[2] += 2;
            delta[a.min(b) + 1] -= 1;
            delta[a + b] -= 1;
            delta[a + b + 1] += 1;
            delta[a.max(b) + limit + 1] += 1;
        }

        let mut res = std::i32::MAX;
        let mut cur = 0;
        for sum in 2..=limit * 2 {
            cur += delta[sum];
            res = res.min(cur);
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 4, 3];
    let limit = 4;
    let res = 1;
    assert_eq!(Solution::min_moves(nums, limit), res);
    let nums = vec![1, 2, 2, 1];
    let limit = 2;
    let res = 2;
    assert_eq!(Solution::min_moves(nums, limit), res);
    let nums = vec![1, 2, 1, 2];
    let limit = 2;
    let res = 0;
    assert_eq!(Solution::min_moves(nums, limit), res);
}
