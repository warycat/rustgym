struct Solution;

use std::collections::HashSet;

impl Solution {
    fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        let mut a = HashSet::new();
        let mut b = HashSet::new();
        for i in left..=right {
            b.insert(i);
        }
        for r in ranges {
            let start = r[0];
            let end = r[1];
            for i in start..=end {
                if b.contains(&i) {
                    a.insert(i);
                }
            }
        }
        a == b
    }
}

#[test]
fn test() {
    let ranges = vec_vec_i32![[1, 2], [3, 4], [5, 6]];
    let left = 2;
    let right = 5;
    let res = true;
    assert_eq!(Solution::is_covered(ranges, left, right), res);
    let ranges = vec_vec_i32![[1, 10], [10, 20]];
    let left = 21;
    let right = 21;
    let res = false;
    assert_eq!(Solution::is_covered(ranges, left, right), res);
}
