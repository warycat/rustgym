struct Solution;

impl Solution {
    fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut end = 0;
        let mut max = 0;
        let mut res = 0;
        for i in 0..n - 1 {
            max = max.max(i + nums[i] as usize);
            if i == end {
                res += 1;
                end = max;
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![2, 3, 1, 1, 4];
    let res = 2;
    assert_eq!(Solution::jump(nums), res);
}
