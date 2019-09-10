struct Solution;

impl Solution {
    fn move_zeroes(nums: &mut Vec<i32>) {
        let mut j = 0;
        for i in 0..nums.len() {
            let x = nums[i];
            if x != 0 {
                nums[i] = 0;
                nums[j] = x;
                j += 1;
            }
        }
    }
}

#[test]
fn test() {
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, vec![1, 3, 12, 0, 0]);
}
