struct Solution;

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left: usize = 0;
        let mut right: usize = nums.len() - 1;
        while left < right {
            if nums[left] + nums[right] == target {
                return vec![(left + 1) as i32, (right + 1) as i32];
            }
            if nums[left] + nums[right] < target {
                left += 1;
                continue;
            }
            if nums[left] + nums[right] > target {
                right -= 1;
                continue;
            }
        }
        vec![0, 0]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
}
