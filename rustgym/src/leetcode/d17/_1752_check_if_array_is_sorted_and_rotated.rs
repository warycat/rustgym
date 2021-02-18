struct Solution;

impl Solution {
    fn check(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut count = 0;
        for i in 0..n {
            if nums[i] > nums[(i + 1) % n] {
                count += 1;
            }
        }
        count < 2
    }
}

#[test]
fn test() {
    let nums = vec![3, 4, 5, 1, 2];
    let res = true;
    assert_eq!(Solution::check(nums), res);
    let nums = vec![2, 1, 3, 4];
    let res = false;
    assert_eq!(Solution::check(nums), res);
    let nums = vec![1, 2, 3];
    let res = true;
    assert_eq!(Solution::check(nums), res);
    let nums = vec![1, 1, 1];
    let res = true;
    assert_eq!(Solution::check(nums), res);
    let nums = vec![2, 1];
    let res = true;
    assert_eq!(Solution::check(nums), res);
}
