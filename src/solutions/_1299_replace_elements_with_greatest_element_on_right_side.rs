struct Solution;
use std::i32;

impl Solution {
    fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let mut max = -1;
        let n = arr.len();
        for i in (0..n).rev() {
            let t = max;
            max = i32::max(arr[i], max);
            arr[i] = t;
        }
        arr
    }
}

#[test]
fn test() {
    let arr = vec![17, 18, 5, 4, 6, 1];
    let res = vec![18, 6, 6, 6, 1, -1];
    assert_eq!(Solution::replace_elements(arr), res);
}
