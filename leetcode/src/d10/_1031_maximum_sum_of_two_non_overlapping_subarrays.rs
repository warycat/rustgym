struct Solution;

use std::i32;

impl Solution {
    fn max_sum_two_no_overlap(mut a: Vec<i32>, l: i32, m: i32) -> i32 {
        let n = a.len();
        let l = l as usize;
        let m = m as usize;
        for i in 1..n {
            a[i] += a[i - 1];
        }
        let mut res = a[l + m - 1];
        let mut max_l = a[l - 1];
        let mut max_m = a[m - 1];
        for i in l + m..n {
            max_l = i32::max(a[i - m] - a[i - m - l], max_l);
            max_m = i32::max(a[i - l] - a[i - l - m], max_m);
            let last_l = a[i] - a[i - l];
            let last_m = a[i] - a[i - m];
            res = i32::max(i32::max(max_m + last_l, max_l + last_m), res);
        }
        res
    }
}

#[test]
fn test() {
    let a = vec![0, 6, 5, 2, 2, 5, 1, 9, 4];
    let l = 1;
    let m = 2;
    let res = 20;
    assert_eq!(Solution::max_sum_two_no_overlap(a, l, m), res);
    let a = vec![3, 8, 1, 3, 2, 1, 8, 9, 0];
    let l = 3;
    let m = 2;
    let res = 29;
    assert_eq!(Solution::max_sum_two_no_overlap(a, l, m), res);
    let a = vec![2, 1, 5, 6, 0, 9, 5, 0, 3, 8];
    let l = 4;
    let m = 3;
    let res = 31;
    assert_eq!(Solution::max_sum_two_no_overlap(a, l, m), res);
}
