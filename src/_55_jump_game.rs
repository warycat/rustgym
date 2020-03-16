struct Solution;

impl Solution {
    fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut last = n - 1;
        for i in (0..n).rev() {
            if i + nums[i] as usize >= last {
                last = i;
            }
        }
        last == 0
    }
}

#[test]
fn test() {
    let nums = vec![2, 3, 1, 1, 4];
    let res = true;
    assert_eq!(Solution::can_jump(nums), res);
    let nums = vec![3, 2, 1, 0, 4];
    let res = false;
    assert_eq!(Solution::can_jump(nums), res);
}
