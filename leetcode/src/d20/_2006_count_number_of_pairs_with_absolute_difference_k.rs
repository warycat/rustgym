struct Solution;

impl Solution {
    fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut res = 0;
        for i in 0..n {
            for j in i + 1..n {
                if (nums[j] - nums[i]).abs() == k {
                    res += 1;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 2, 1];
    let k = 1;
    let res = 4;
    assert_eq!(Solution::count_k_difference(nums, k), res);
    let nums = vec![1, 3];
    let k = 3;
    let res = 0;
    assert_eq!(Solution::count_k_difference(nums, k), res);
    let nums = vec![3, 2, 1, 5, 4];
    let k = 2;
    let res = 3;
    assert_eq!(Solution::count_k_difference(nums, k), res);
}
