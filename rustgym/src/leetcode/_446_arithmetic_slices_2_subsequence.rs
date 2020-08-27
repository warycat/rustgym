struct Solution;

use std::collections::HashMap;

impl Solution {
    fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
        let n = a.len();
        let mut dp: HashMap<(usize, i64), usize> = HashMap::new();
        let mut res = 0;
        for i in 0..n {
            for j in 0..i {
                let diff = a[i] as i64 - a[j] as i64;
                let prev = *dp.entry((j, diff)).or_insert(1);
                res += prev - 1;
                *dp.entry((i, diff)).or_insert(1) += prev;
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let a = vec![2, 4, 6, 8, 10];
    let res = 7;
    assert_eq!(Solution::number_of_arithmetic_slices(a), res);
    let a = vec![2, 2, 3, 4];
    let res = 2;
    assert_eq!(Solution::number_of_arithmetic_slices(a), res);
    let a = vec![0, 2000000000, -294967296];
    let res = 0;
    assert_eq!(Solution::number_of_arithmetic_slices(a), res);
}
