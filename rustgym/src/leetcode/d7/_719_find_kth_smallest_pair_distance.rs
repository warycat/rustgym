struct Solution;

impl Solution {
    fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        nums.sort_unstable();
        let mut lo = 0;
        let mut hi = nums[n - 1] - nums[0];
        while lo < hi {
            let mi = lo + (hi - lo) / 2;
            let mut count = 0;
            let mut j = 1;
            for i in 0..n - 1 {
                while j < n && nums[j] - nums[i] <= mi {
                    j += 1;
                }
                count += j - 1 - i;
            }
            if count >= k as usize {
                hi = mi;
            } else {
                lo = mi + 1;
            }
        }
        lo
    }
}

#[test]
fn test() {
    let nums = vec![1, 3, 1];
    let k = 1;
    let res = 0;
    assert_eq!(Solution::smallest_distance_pair(nums, k), res);
    let nums = vec![62, 100, 4];
    let k = 2;
    let res = 58;
    assert_eq!(Solution::smallest_distance_pair(nums, k), res);
}
