struct Solution;

use std::collections::BTreeMap;

impl Solution {
    fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut count: BTreeMap<i32, usize> = BTreeMap::new();
        let n = nums.len();
        let mut res = vec![0; n];
        for i in (0..n).rev() {
            let mut sum = 0;
            let x = nums[i];
            for (_, v) in count.range(..x) {
                sum += v;
            }
            *count.entry(x).or_default() += 1;
            res[i] = sum as i32;
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![5, 2, 6, 1];
    let res = vec![2, 1, 1, 0];
    assert_eq!(Solution::count_smaller(nums), res);
}
