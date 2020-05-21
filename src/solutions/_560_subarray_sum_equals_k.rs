struct Solution;

use std::collections::HashMap;

impl Solution {
    fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        let mut hm: HashMap<i32, i32> = HashMap::new();
        let mut res = 0;
        hm.insert(0, 1);
        for x in nums {
            sum += x;
            res += hm.get(&(sum - k)).unwrap_or(&0);
            *hm.entry(sum).or_default() += 1;
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 1];
    let k = 2;
    assert_eq!(Solution::subarray_sum(nums, k), 2);
    let nums = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let k = 0;
    assert_eq!(Solution::subarray_sum(nums, k), 55);
}
