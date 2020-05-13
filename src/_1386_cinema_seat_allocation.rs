#![allow(clippy::unreadable_literal)]
struct Solution;
use std::collections::HashMap;

impl Solution {
    fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut hm: HashMap<usize, u16> = HashMap::new();
        for seat in reserved_seats {
            let i = (seat[0] - 1) as usize;
            let j = (seat[1] - 1) as usize;
            *hm.entry(i).or_default() |= 1 << j;
        }
        let mut res = 0;
        for &row_bitset in hm.values() {
            res += Self::num_of_families(row_bitset);
        }
        res += (n - hm.len()) * 2;
        res as i32
    }

    fn num_of_families(row_bitset: u16) -> usize {
        let two = 0b0111111110;
        let mid = 0b0001111000;
        let left = 0b0111100000;
        let right = 0b0000011110;
        if row_bitset & two == 0 {
            2
        } else {
            if row_bitset & mid == 0 {
                1
            } else {
                if row_bitset & left == 0 || row_bitset & right == 0 {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[test]
fn test() {
    let n = 3;
    let reserved_seats = vec_vec_i32![[1, 2], [1, 3], [1, 8], [2, 6], [3, 1], [3, 10]];
    let res = 4;
    assert_eq!(Solution::max_number_of_families(n, reserved_seats), res);
    let n = 2;
    let reserved_seats = vec_vec_i32![[2, 1], [1, 8], [2, 6]];
    let res = 2;
    assert_eq!(Solution::max_number_of_families(n, reserved_seats), res);
    let n = 4;
    let reserved_seats = vec_vec_i32![[4, 3], [1, 4], [4, 6], [1, 7]];
    let res = 4;
    assert_eq!(Solution::max_number_of_families(n, reserved_seats), res);
}
