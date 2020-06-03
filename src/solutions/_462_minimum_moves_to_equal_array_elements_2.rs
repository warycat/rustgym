struct Solution;

impl Solution {
    fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let median = nums[n / 2];
        nums.into_iter().map(|x| (x - median).abs()).sum()
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3];
    let res = 2;
    assert_eq!(Solution::min_moves2(nums), res);
}
