struct Solution;

use std::collections::HashMap;

impl Solution {
    fn max_non_overlapping(nums: Vec<i32>, target: i32) -> i32 {
        let mut sum = 0;
        let mut hm: HashMap<i32, usize> = HashMap::new();
        let mut last = 0;
        hm.insert(0, 0);
        let n = nums.len();
        let mut res = 0;
        for i in 0..n {
            let x = nums[i];
            sum += x;
            if let Some(&j) = hm.get(&(sum - target)) {
                if j >= last {
                    res += 1;
                    last = i + 1;
                }
            }
            *hm.entry(sum).or_default() = i + 1;
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 1, 1, 1];
    let target = 2;
    let res = 2;
    assert_eq!(Solution::max_non_overlapping(nums, target), res);
    let nums = vec![-1, 3, 5, 1, 4, 2, -9];
    let target = 6;
    let res = 2;
    assert_eq!(Solution::max_non_overlapping(nums, target), res);
    let nums = vec![-2, 6, 6, 3, 5, 4, 1, 2, 8];
    let target = 10;
    let res = 3;
    assert_eq!(Solution::max_non_overlapping(nums, target), res);
    let nums = vec![0, 0, 0];
    let target = 0;
    let res = 3;
    assert_eq!(Solution::max_non_overlapping(nums, target), res);
}
