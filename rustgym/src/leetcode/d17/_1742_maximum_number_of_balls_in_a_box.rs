struct Solution;

use std::collections::HashMap;

impl Solution {
    fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut count = HashMap::new();
        for i in low_limit..=high_limit {
            *count.entry(Self::sum(i)).or_default() += 1;
        }
        *count.values().max().unwrap()
    }

    fn sum(mut x: i32) -> i32 {
        let mut res = 0;
        while x > 0 {
            res += x % 10;
            x /= 10;
        }
        res
    }
}

#[test]
fn test() {
    let low_limit = 1;
    let high_limit = 10;
    let res = 2;
    assert_eq!(Solution::count_balls(low_limit, high_limit), res);
    let low_limit = 5;
    let high_limit = 15;
    let res = 2;
    assert_eq!(Solution::count_balls(low_limit, high_limit), res);
    let low_limit = 19;
    let high_limit = 28;
    let res = 2;
    assert_eq!(Solution::count_balls(low_limit, high_limit), res);
}
