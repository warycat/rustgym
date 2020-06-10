struct Solution;
use std::collections::HashSet;

impl Solution {
    fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut mask = 0;
        for i in (0..30).rev() {
            mask |= 1 << i;
            let mut hs: HashSet<i32> = HashSet::new();
            for &x in &nums {
                hs.insert(x & mask);
            }
            let tmp = max | (1 << i);
            for &x in &hs {
                if hs.contains(&(tmp ^ x)) {
                    max = tmp;
                    break;
                }
            }
        }
        max
    }
}

#[test]
fn test() {
    let nums = vec![3, 10, 5, 25, 2, 8];
    let res = 28;
    assert_eq!(Solution::find_maximum_xor(nums), res);
}
