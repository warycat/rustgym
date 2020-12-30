struct Solution;

impl Solution {
    fn wiggle_sort(nums: &mut Vec<i32>) {
        let n = nums.len();
        for i in 0..n {
            if i % 2 == 1 {
                if nums[i] < nums[i - 1] {
                    nums.swap(i, i - 1);
                }
            } else {
                if i > 0 && nums[i] > nums[i - 1] {
                    nums.swap(i, i - 1);
                }
            }
        }
    }
}

#[test]
fn test() {
    let mut nums = vec![3, 5, 2, 1, 6, 4];
    Solution::wiggle_sort(&mut nums);
    let res = vec![3, 5, 1, 6, 2, 4];
    assert_eq!(nums, res);
}
