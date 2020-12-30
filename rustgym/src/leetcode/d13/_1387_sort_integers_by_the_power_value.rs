struct Solution;
use std::collections::BinaryHeap;

impl Solution {
    fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let k = k as usize;
        let mut pq: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        for i in lo..=hi {
            pq.push((Self::power(i), i));
            if pq.len() > k {
                pq.pop();
            }
        }
        pq.pop().unwrap().1
    }
    fn power(mut num: i32) -> i32 {
        let mut res = 0;
        while num != 1 {
            res += 1;
            if num % 2 == 0 {
                num /= 2;
            } else {
                num = 3 * num + 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let lo = 12;
    let hi = 15;
    let k = 2;
    let res = 13;
    assert_eq!(Solution::get_kth(lo, hi, k), res);
    let lo = 1;
    let hi = 1;
    let k = 1;
    let res = 1;
    assert_eq!(Solution::get_kth(lo, hi, k), res);
    let lo = 7;
    let hi = 11;
    let k = 4;
    let res = 7;
    assert_eq!(Solution::get_kth(lo, hi, k), res);
    let lo = 10;
    let hi = 20;
    let k = 5;
    let res = 13;
    assert_eq!(Solution::get_kth(lo, hi, k), res);
    let lo = 1;
    let hi = 1000;
    let k = 777;
    let res = 570;
    assert_eq!(Solution::get_kth(lo, hi, k), res);
}
