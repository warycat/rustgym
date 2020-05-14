struct Solution;

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    fn count_prime_set_bits(l: i32, r: i32) -> i32 {
        let mut res = 0;
        let hs: HashSet<i32> = HashSet::from_iter(vec![2, 3, 5, 7, 11, 13, 17, 19]);
        for i in l..=r {
            let ones = i.count_ones() as i32;
            if hs.contains(&ones) {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_prime_set_bits(6, 10), 4);
    assert_eq!(Solution::count_prime_set_bits(10, 15), 5);
}
