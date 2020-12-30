struct Solution;

use std::cmp::Ordering::*;

impl Solution {
    fn find_kth_bit(n: i32, k: i32) -> char {
        if Self::find(n, k - 1) == 0 {
            '0'
        } else {
            '1'
        }
    }

    fn find(n: i32, k: i32) -> i32 {
        if n == 1 {
            0
        } else {
            let size = (1 << n) - 1;
            match (size / 2).cmp(&k) {
                Equal => 1,
                Greater => Self::find(n - 1, k),
                Less => 1 - Self::find(n - 1, size - 1 - k),
            }
        }
    }
}

#[test]
fn test() {
    let n = 3;
    let k = 1;
    let res = '0';
    assert_eq!(Solution::find_kth_bit(n, k), res);
    let n = 4;
    let k = 11;
    let res = '1';
    assert_eq!(Solution::find_kth_bit(n, k), res);
    let n = 1;
    let k = 1;
    let res = '0';
    assert_eq!(Solution::find_kth_bit(n, k), res);
    let n = 2;
    let k = 3;
    let res = '1';
    assert_eq!(Solution::find_kth_bit(n, k), res);
    let n = 3;
    let k = 7;
    let res = '1';
    assert_eq!(Solution::find_kth_bit(n, k), res);
}
