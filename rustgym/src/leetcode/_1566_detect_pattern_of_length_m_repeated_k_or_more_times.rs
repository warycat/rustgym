struct Solution;

impl Solution {
    fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        let m = m as usize;
        let k = k as usize;
        arr.windows(m * k)
            .any(|w| w.chunks(m).all(|v| *v == w[0..m]))
    }
}

#[test]
fn test() {
    let arr = vec![1, 2, 4, 4, 4, 4];
    let m = 1;
    let k = 3;
    let res = true;
    assert_eq!(Solution::contains_pattern(arr, m, k), res);
    let arr = vec![1, 2, 1, 2, 1, 1, 1, 3];
    let m = 2;
    let k = 2;
    let res = true;
    assert_eq!(Solution::contains_pattern(arr, m, k), res);
    let arr = vec![1, 2, 1, 2, 1, 3];
    let m = 2;
    let k = 3;
    let res = false;
    assert_eq!(Solution::contains_pattern(arr, m, k), res);
    let arr = vec![1, 2, 3, 1, 2];
    let m = 2;
    let k = 2;
    let res = false;
    assert_eq!(Solution::contains_pattern(arr, m, k), res);
    let arr = vec![2, 2, 2, 2];
    let m = 2;
    let k = 3;
    let res = false;
    assert_eq!(Solution::contains_pattern(arr, m, k), res);
    let arr = vec![2, 2];
    let m = 1;
    let k = 2;
    let res = true;
    assert_eq!(Solution::contains_pattern(arr, m, k), res);
}
