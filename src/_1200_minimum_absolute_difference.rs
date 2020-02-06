struct Solution;

use std::i32;

impl Solution {
    fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();
        let min = arr
            .windows(2)
            .fold(i32::MAX, |x, v| i32::min(x, v[1] - v[0]));
        let mut res: Vec<Vec<i32>> = vec![];
        for v in arr.windows(2) {
            if v[1] - v[0] == min {
                res.push(v.to_vec())
            }
        }
        res
    }
}

#[test]
fn test() {
    let arr = vec![4, 2, 1, 3];
    let res: Vec<Vec<i32>> = vec_vec_i32![[1, 2], [2, 3], [3, 4]];
    assert_eq!(Solution::minimum_abs_difference(arr), res);
    let arr = vec![1, 3, 6, 10, 15];
    let res: Vec<Vec<i32>> = vec_vec_i32![[1, 3]];
    assert_eq!(Solution::minimum_abs_difference(arr), res);
    let arr = vec![3, 8, -10, 23, 19, -4, -14, 27];
    let res: Vec<Vec<i32>> = vec_vec_i32![[-14, -10], [19, 23], [23, 27]];
    assert_eq!(Solution::minimum_abs_difference(arr), res);
}
