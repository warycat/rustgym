struct Solution;
use std::collections::HashMap;

impl Solution {
    fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut count: HashMap<i32, usize> = HashMap::new();
        for x in nums {
            *count.entry(x).or_default() += 1;
        }
        let mut res = 0;
        for &key in count.keys() {
            let a = count.get(&key).unwrap_or(&0);
            let b = count.get(&(k - key)).unwrap_or(&0);
            res += a.min(b);
        }
        (res / 2) as i32
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 3, 4];
    let k = 5;
    let res = 2;
    assert_eq!(Solution::max_operations(nums, k), res);
    let nums = vec![3, 1, 3, 4, 3];
    let k = 6;
    let res = 1;
    assert_eq!(Solution::max_operations(nums, k), res);
}
