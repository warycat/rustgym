struct Solution;
use std::collections::HashMap;

impl Solution {
    fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut hm: HashMap<i32, usize> = HashMap::new();
        let mut res = 0;
        for x in nums {
            let count = hm.entry(x).or_default();
            res += *count;
            *count += 1;
        }
        res as i32
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 1, 1, 3];
    let res = 4;
    assert_eq!(Solution::num_identical_pairs(nums), res);
    let nums = vec![1, 1, 1, 1];
    let res = 6;
    assert_eq!(Solution::num_identical_pairs(nums), res);
    let nums = vec![1, 2, 3];
    let res = 0;
    assert_eq!(Solution::num_identical_pairs(nums), res);
}
