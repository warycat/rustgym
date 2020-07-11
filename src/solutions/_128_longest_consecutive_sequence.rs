struct Solution;
use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let hs: HashSet<i32> = HashSet::from_iter(nums);
        let mut res = 0;
        for &x in &hs {
            if hs.contains(&(x - 1)) {
                continue;
            }
            let mut i = 1;
            while hs.contains(&(x + i)) {
                i += 1;
                res = res.max(i);
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![100, 4, 200, 1, 3, 2];
    let res = 4;
    assert_eq!(Solution::longest_consecutive(nums), res);
}
