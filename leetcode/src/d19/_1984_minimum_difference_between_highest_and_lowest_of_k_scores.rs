struct Solution;

impl Solution {
    fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        nums.sort_unstable();
        let mut min = std::i32::MAX;
        for w in nums.windows(k) {
            min = min.min(w[k - 1] - w[0]);
        }
        min
    }
}

#[test]
fn test() {
    let nums = vec![90];
    let k = 1;
    let res = 0;
    assert_eq!(Solution::minimum_difference(nums, k), res);
    let nums = vec![9, 4, 1, 7];
    let k = 2;
    let res = 2;
    assert_eq!(Solution::minimum_difference(nums, k), res);
}
