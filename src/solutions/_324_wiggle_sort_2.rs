struct Solution;

impl Solution {
    fn wiggle_sort(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut sorted = nums.to_vec();
        sorted.sort_unstable();
        let k = if n % 2 == 0 { n / 2 } else { n / 2 + 1 };
        for i in 0..k {
            nums[i * 2] = sorted[k - 1 - i];
        }
        for i in 0..(n - k) {
            nums[i * 2 + 1] = sorted[n - 1 - i];
        }
    }
}

#[test]
fn test() {
    let mut nums = vec![1, 5, 1, 1, 6, 4];
    let res = vec![1, 6, 1, 5, 1, 4];
    Solution::wiggle_sort(&mut nums);
    assert_eq!(nums, res);
    let mut nums = vec![1, 1, 2, 1, 2, 2, 1];
    let res = vec![1, 2, 1, 2, 1, 2, 1];
    Solution::wiggle_sort(&mut nums);
    assert_eq!(nums, res);
    let mut nums = vec![4, 5, 5, 6];
    let res = vec![5, 6, 4, 5];
    Solution::wiggle_sort(&mut nums);
    assert_eq!(nums, res);
}
