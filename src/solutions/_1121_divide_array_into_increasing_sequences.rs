struct Solution;

impl Solution {
    fn can_divide_into_subsequences(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let mut cur = 1;
        let mut groups = 1;
        for i in 1..n {
            if nums[i] == nums[i - 1] {
                cur += 1;
            } else {
                cur = 1;
            }
            groups = groups.max(cur);
        }
        n >= k as usize * groups
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 2, 3, 3, 4, 4];
    let k = 3;
    let res = true;
    assert_eq!(Solution::can_divide_into_subsequences(nums, k), res);
    let nums = vec![5, 6, 6, 7, 8];
    let k = 3;
    let res = false;
    assert_eq!(Solution::can_divide_into_subsequences(nums, k), res);
}
