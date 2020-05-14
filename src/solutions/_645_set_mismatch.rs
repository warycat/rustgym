struct Solution;

use std::collections::HashSet;

impl Solution {
    fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut hs: HashSet<i32> = HashSet::new();
        let mut res: Vec<i32> = vec![];
        for x in nums {
            if !hs.insert(x) {
                res.push(x);
            }
        }
        for i in 1..=n {
            if hs.insert(i as i32) {
                res.push(i as i32);
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 2, 4];
    let res = vec![2, 3];
    assert_eq!(Solution::find_error_nums(nums), res);
}
