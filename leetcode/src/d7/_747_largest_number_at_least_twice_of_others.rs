struct Solution;

impl Solution {
    fn dominant_index(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return 0;
        }
        let mut copy = nums.clone();
        copy.sort_unstable();
        if copy[n - 1] < 2 * copy[n - 2] {
            return -1;
        }
        for i in 0..n {
            if copy[n - 1] == nums[i] {
                return i as i32;
            }
        }
        -1
    }
}

#[test]
fn test() {
    let nums = vec![3, 6, 1, 0];
    assert_eq!(Solution::dominant_index(nums), 1);
    let nums = vec![1, 2, 3, 4];
    assert_eq!(Solution::dominant_index(nums), -1);
}
