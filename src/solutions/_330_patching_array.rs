struct Solution;

impl Solution {
    fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut res = 0;
        let mut miss: i64 = 1;
        let mut i = 0;
        while miss <= n as i64 {
            if i < nums.len() && nums[i] as i64 <= miss {
                miss += nums[i] as i64;
                i += 1;
            } else {
                miss += miss;
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 3];
    let n = 6;
    let res = 1;
    assert_eq!(Solution::min_patches(nums, n), res);
    let nums = vec![1, 5, 10];
    let n = 20;
    let res = 2;
    assert_eq!(Solution::min_patches(nums, n), res);
    let nums = vec![1, 2, 2];
    let n = 5;
    let res = 0;
    assert_eq!(Solution::min_patches(nums, n), res);
}
