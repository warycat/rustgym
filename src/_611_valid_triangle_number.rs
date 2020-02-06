struct Solution;

impl Solution {
    fn triangle_number(mut nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let n = nums.len();
        nums.sort_unstable();
        for i in (2..n).rev() {
            let mut l = 0;
            let mut r = i - 1;
            while l < r {
                if nums[l] + nums[r] > nums[i] {
                    res += r - l;
                    r -= 1;
                } else {
                    l += 1;
                }
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let nums = vec![2, 2, 3, 4];
    let res = 3;
    assert_eq!(Solution::triangle_number(nums), res);
}
