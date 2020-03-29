struct Solution;

impl Solution {
    fn three_sum_smaller(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let mut res = 0;
        for i in 0..n {
            let mut j = i + 1;
            let mut k = n - 1;
            while j < k {
                if nums[i] + nums[j] + nums[k] < target {
                    res += k - j;
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let nums = vec![-2, 0, 1, 3];
    let target = 2;
    let res = 2;
    assert_eq!(Solution::three_sum_smaller(nums, target), res);
}
