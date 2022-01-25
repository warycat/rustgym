struct Solution;

impl Solution {
    fn smallest_equal(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for i in 0..n {
            if i as i32 % 10 == nums[i] {
                return i as i32;
            }
        }
        -1
    }
}

#[test]
fn test() {
    let nums = vec![0, 1, 2];
    let res = 0;
    assert_eq!(Solution::smallest_equal(nums), res);
    let nums = vec![4, 3, 2, 1];
    let res = 2;
    assert_eq!(Solution::smallest_equal(nums), res);
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let res = -1;
    assert_eq!(Solution::smallest_equal(nums), res);
}
