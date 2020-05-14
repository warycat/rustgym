struct Solution;

use std::collections::HashMap;

impl Solution {
    fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut hs: HashMap<i32, i32> = HashMap::new();
        let mut max = 0;
        for &x in &nums {
            let e = hs.entry(x).or_default();
            *e += 1;
        }
        for (x, u) in &hs {
            if let Some(v) = hs.get(&(x - 1)) {
                max = i32::max(u + v, max);
            }
        }
        max
    }
}

#[test]
fn test() {
    let nums = vec![1, 3, 2, 2, 5, 2, 3, 7];
    let res = 5;
    assert_eq!(Solution::find_lhs(nums), res);
}
