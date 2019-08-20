struct Solution;

impl Solution {
    fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.chunks(2).fold(0, |sum, pair| sum + pair[0])
    }
}

#[test]
fn test() {
    let nums = vec![1, 4, 3, 2];
    assert_eq!(Solution::array_pair_sum(nums), 4);
}
