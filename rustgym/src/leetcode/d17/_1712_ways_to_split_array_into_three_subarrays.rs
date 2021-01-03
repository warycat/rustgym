struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn ways_to_split(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut prefix: Vec<i32> = vec![];
        let mut prev = 0;
        let mut res = 0;
        for i in 0..n {
            prev += nums[i];
            prefix.push(prev);
        }
        let total = prev;
        let mut mid_start = 0;
        let mut mid_end = 0;
        for i in 0..n {
            let left = prefix[i];
            mid_start = mid_start.max(i + 1);
            while mid_start < n && prefix[mid_start] - left < left {
                mid_start += 1;
            }
            while mid_end + 1 < n && total - prefix[mid_end] >= prefix[mid_end] - left {
                mid_end += 1;
            }
            if mid_end >= mid_start {
                res += (mid_end - mid_start) as i64;
                res %= MOD;
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 1];
    let res = 1;
    assert_eq!(Solution::ways_to_split(nums), res);
    let nums = vec![1, 2, 2, 2, 5, 0];
    let res = 3;
    assert_eq!(Solution::ways_to_split(nums), res);
    let nums = vec![3, 2, 1];
    let res = 0;
    assert_eq!(Solution::ways_to_split(nums), res);
    let nums = vec![0, 3, 3];
    let res = 1;
    assert_eq!(Solution::ways_to_split(nums), res);
}
