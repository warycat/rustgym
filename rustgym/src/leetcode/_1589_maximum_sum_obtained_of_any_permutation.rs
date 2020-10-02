struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn max_sum_range_query(mut nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut count = vec![0; n + 1];
        for r in requests {
            count[r[0] as usize] += 1;
            count[r[1] as usize + 1] -= 1;
        }
        count.pop();
        let mut prev = 0;
        for i in 0..n {
            prev += count[i];
            count[i] = prev;
        }
        count.sort_unstable();
        nums.sort_unstable();
        let mut res = 0;
        for i in 0..n {
            res += nums[i] as i64 * count[i] as i64;
            res %= MOD;
        }
        res as i32
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 4, 5];
    let requests = vec_vec_i32![[1, 3], [0, 1]];
    let res = 19;
    assert_eq!(Solution::max_sum_range_query(nums, requests), res);
    let nums = vec![1, 2, 3, 4, 5, 6];
    let requests = vec_vec_i32![[0, 1]];
    let res = 11;
    assert_eq!(Solution::max_sum_range_query(nums, requests), res);
    let nums = vec![1, 2, 3, 4, 5, 10];
    let requests = vec_vec_i32![[0, 2], [1, 3], [1, 1]];
    let res = 47;
    assert_eq!(Solution::max_sum_range_query(nums, requests), res);
}
