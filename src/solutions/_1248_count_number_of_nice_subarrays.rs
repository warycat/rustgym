struct Solution;

impl Solution {
    fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut count = vec![0; n + 1];
        let k = k as usize;
        count[0] = 1;
        let mut prev = 0;
        let mut res = 0;
        for i in 0..n {
            if nums[i] % 2 == 1 {
                prev += 1;
            }
            count[prev] += 1;
            if prev >= k {
                res += count[prev - k];
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 2, 1, 1];
    let k = 3;
    let res = 2;
    assert_eq!(Solution::number_of_subarrays(nums, k), res);
    let nums = vec![2, 4, 6];
    let k = 1;
    let res = 0;
    assert_eq!(Solution::number_of_subarrays(nums, k), res);
    let nums = vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2];
    let k = 2;
    let res = 16;
    assert_eq!(Solution::number_of_subarrays(nums, k), res);
}
