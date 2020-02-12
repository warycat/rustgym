struct Solution;

impl Solution {
    fn find_min(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut h = nums.len() - 1;
        while l < h {
            if nums[l] < nums[h] {
                return nums[l];
            }
            let m = l + (h - l) / 2;
            if nums[l] <= nums[m] {
                l = m + 1;
            } else {
                h = m;
            }
        }
        nums[l]
    }
}

#[test]
fn test() {
    let nums = vec![3, 4, 5, 1, 2];
    let res = 1;
    assert_eq!(Solution::find_min(nums), res);
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let res = 0;
    assert_eq!(Solution::find_min(nums), res);
}
