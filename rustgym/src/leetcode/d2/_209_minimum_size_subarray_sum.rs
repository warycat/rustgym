struct Solution;

impl Solution {
    fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = std::usize::MAX;
        let mut sum = 0;
        let mut l = 0;
        for r in 0..n {
            sum += nums[r];
            while sum >= s {
                res = usize::min(r - l + 1, res);
                sum -= nums[l];
                l += 1;
            }
        }
        if res == usize::MAX {
            0
        } else {
            res as i32
        }
    }
}

#[test]
fn test() {
    let s = 7;
    let nums = vec![2, 3, 1, 2, 4, 3];
    let res = 2;
    assert_eq!(Solution::min_sub_array_len(s, nums), res);
    let s = 4;
    let nums = vec![1, 4, 4];
    let res = 1;
    assert_eq!(Solution::min_sub_array_len(s, nums), res);
}
