struct Solution;
use std::collections::HashMap;

impl Solution {
    fn max_equal_freq(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        let mut count: HashMap<i32, usize> = HashMap::new();
        let mut freq: HashMap<usize, usize> = HashMap::new();
        let mut max_freq = 0;
        for i in 0..n {
            let f = count.entry(nums[i]).or_default();
            if *f != 0 {
                *freq.entry(*f).or_default() -= 1;
            }
            *f += 1;
            *freq.entry(*f).or_default() += 1;
            max_freq = max_freq.max(*f);
            if max_freq == 1
                || *freq.entry(max_freq).or_default() * max_freq == i
                || (*freq.entry(max_freq - 1).or_default() + 1) * (max_freq - 1) == i
            {
                res = res.max(i + 1);
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let nums = vec![2, 2, 1, 1, 5, 3, 3, 5];
    let res = 7;
    assert_eq!(Solution::max_equal_freq(nums), res);
    let nums = vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5];
    let res = 13;
    assert_eq!(Solution::max_equal_freq(nums), res);
    let nums = vec![1, 1, 1, 2, 2, 2];
    let res = 5;
    assert_eq!(Solution::max_equal_freq(nums), res);
}
