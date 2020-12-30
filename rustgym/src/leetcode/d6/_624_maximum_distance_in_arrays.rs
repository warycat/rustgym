struct Solution;

use std::i32;

impl Solution {
    fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let n = arrays.len();
        let mut min = arrays[0][0];
        let mut max = arrays[0][arrays[0].len() - 1];
        let mut res = 0;
        for i in 1..n {
            let a = &arrays[i];
            let m = a.len();
            res = i32::max(i32::max(max - a[0], a[m - 1] - min), res);
            min = i32::min(a[0], min);
            max = i32::max(a[m - 1], max);
        }
        res
    }
}

#[test]
fn test() {
    let arrays: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]];
    assert_eq!(Solution::max_distance(arrays), 4);
}
