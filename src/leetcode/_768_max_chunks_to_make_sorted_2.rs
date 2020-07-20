struct Solution;

impl Solution {
    fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut left = vec![0; n];
        let mut right = vec![0; n];
        let mut max = 0;
        for i in 0..n {
            max = max.max(arr[i]);
            left[i] = max;
        }
        let mut min = std::i32::MAX;
        for i in (0..n).rev() {
            min = min.min(arr[i]);
            right[i] = min;
        }
        let mut res = 1;
        for i in 0..n - 1 {
            if left[i] <= right[i + 1] {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let arr = vec![5, 4, 3, 2, 1];
    let res = 1;
    assert_eq!(Solution::max_chunks_to_sorted(arr), res);
    let arr = vec![2, 1, 3, 4, 4];
    let res = 4;
    assert_eq!(Solution::max_chunks_to_sorted(arr), res);
}
