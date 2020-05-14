struct Solution;

use std::collections::HashMap;

impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        if a == 0 {
            b
        } else {
            Self::gcd(b % a, a)
        }
    }

    fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        let mut max = 0;
        for x in deck {
            let count = hm.entry(x).or_default();
            *count += 1;
        }
        for &v in hm.values() {
            max = Self::gcd(max, v);
        }
        max >= 2
    }
}

#[test]
fn test() {
    let deck = vec![1, 2, 3, 4, 4, 3, 2, 1];
    assert_eq!(Solution::has_groups_size_x(deck), true);
    let deck = vec![1, 1, 1, 2, 2, 2, 3, 3];
    assert_eq!(Solution::has_groups_size_x(deck), false);
    let deck = vec![1];
    assert_eq!(Solution::has_groups_size_x(deck), false);
    let deck = vec![1, 1];
    assert_eq!(Solution::has_groups_size_x(deck), true);
    let deck = vec![1, 1, 2, 2, 2, 2];
    assert_eq!(Solution::has_groups_size_x(deck), true);
}
