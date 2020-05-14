struct Solution;

impl Solution {
    fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut l = 0;
        let mut r = n - 1;
        while l < r {
            let mut m = l + (r - l) / 2;
            if m % 2 == 1 {
                m -= 1;
            }
            if nums[m + 1] == nums[m] {
                l = m + 2;
            } else {
                r = m;
            }
        }
        nums[l]
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 2, 3, 3, 4, 4, 8, 8];
    let res = 2;
    assert_eq!(Solution::single_non_duplicate(nums), res);
    let nums = vec![3, 3, 7, 7, 10, 11, 11];
    let res = 10;
    assert_eq!(Solution::single_non_duplicate(nums), res);
}
