struct Solution;

impl Solution {
    fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut prev: Option<usize> = None;
        let k = k as usize;
        let n = nums.len();
        for i in 0..n {
            if nums[i] == 1 {
                if let Some(j) = prev {
                    if i - j <= k {
                        return false;
                    }
                }
                prev = Some(i)
            }
        }
        true
    }
}

#[test]
fn test() {
    let nums = vec![1, 0, 0, 0, 1, 0, 0, 1];
    let k = 2;
    let res = true;
    assert_eq!(Solution::k_length_apart(nums, k), res);
    let nums = vec![1, 0, 0, 1, 0, 1];
    let k = 2;
    let res = false;
    assert_eq!(Solution::k_length_apart(nums, k), res);
    let nums = vec![1, 1, 1, 1, 1];
    let k = 0;
    let res = true;
    assert_eq!(Solution::k_length_apart(nums, k), res);
    let nums = vec![0, 1, 0, 1];
    let k = 1;
    let res = true;
    assert_eq!(Solution::k_length_apart(nums, k), res);
}
