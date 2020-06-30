struct Solution;

const MOD: i32 = 1_000_000_007;

impl Solution {
    fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut res = 0;
        let n = nums.len();
        let mut power = vec![];
        let mut prev = 1;
        for _ in 0..n {
            power.push(prev);
            prev *= 2;
            prev %= MOD;
        }
        let mut l = 0;
        let mut r = n - 1;
        while l <= r {
            if nums[l] + nums[r] <= target {
                res += power[r - l];
                res %= MOD;
                l += 1;
            } else {
                if r == 0 {
                    break;
                }
                r -= 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![3, 5, 6, 7];
    let target = 9;
    let res = 4;
    assert_eq!(Solution::num_subseq(nums, target), res);
}
