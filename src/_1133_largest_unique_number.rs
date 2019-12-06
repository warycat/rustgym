struct Solution;

use std::collections::BTreeMap;

impl Solution {
    fn largest_unique_number(a: Vec<i32>) -> i32 {
        let mut btm: BTreeMap<i32, i32> = BTreeMap::new();
        for x in a {
            *btm.entry(x).or_default() += 1;
        }
        for (&k, &v) in btm.iter().rev() {
            if v == 1 {
                return k;
            }
        }
        -1
    }
}

#[test]
fn test() {
    let a = vec![5, 7, 3, 9, 4, 9, 8, 3, 1];
    assert_eq!(Solution::largest_unique_number(a), 8);
    let a = vec![9, 9, 8, 8];
    assert_eq!(Solution::largest_unique_number(a), -1);
}
