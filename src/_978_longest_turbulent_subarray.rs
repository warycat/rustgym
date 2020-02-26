struct Solution;
use std::cmp::Ordering::*;

impl Solution {
    fn max_turbulence_size(a: Vec<i32>) -> i32 {
        let n = a.len();
        let mut res = 1;
        let mut inc = 1;
        let mut dec = 1;
        for i in 1..n {
            match (a[i] - a[i - 1]).cmp(&0) {
                Equal => {
                    inc = 1;
                    dec = 1;
                }
                Less => {
                    inc = dec + 1;
                    dec = 1;
                }
                Greater => {
                    dec = inc + 1;
                    inc = 1;
                }
            }
            res = res.max(inc.max(dec));
        }
        res
    }
}

#[test]
fn test() {
    let a = vec![9, 4, 2, 10, 7, 8, 8, 1, 9];
    let res = 5;
    assert_eq!(Solution::max_turbulence_size(a), res);
    let a = vec![4, 8, 12, 16];
    let res = 2;
    assert_eq!(Solution::max_turbulence_size(a), res);
    let a = vec![100];
    let res = 1;
    assert_eq!(Solution::max_turbulence_size(a), res);
    let a = vec![100, 100, 100];
    let res = 1;
    assert_eq!(Solution::max_turbulence_size(a), res);
}
