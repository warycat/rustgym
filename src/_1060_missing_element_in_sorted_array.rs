struct Solution;

impl Solution {
    fn missing_element(nums: Vec<i32>, k: i32) -> i32 {
        let mut l = 0;
        let mut h = nums.len();
        while l < h {
            let m = l + (h - l) / 2;
            if nums[m] - nums[0] < k + m as i32 {
                l = m + 1;
            } else {
                h = m;
            }
        }
        nums[0] + l as i32 + k - 1
    }
}

#[test]
fn test() {
    let nums = vec![4, 7, 9, 10];
    let k = 1;
    let res = 5;
    assert_eq!(Solution::missing_element(nums, k), res);
    let nums = vec![4, 7, 9, 10];
    let k = 3;
    let res = 8;
    assert_eq!(Solution::missing_element(nums, k), res);
    let nums = vec![1, 2, 4];
    let k = 3;
    let res = 6;
    assert_eq!(Solution::missing_element(nums, k), res);
}
