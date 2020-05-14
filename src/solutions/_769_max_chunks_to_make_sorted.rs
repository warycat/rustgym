struct Solution;

impl Solution {
    fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut max = 0;
        let n = arr.len();
        let mut res = 0;
        for i in 0..n {
            max = max.max(arr[i]);
            if max == i as i32 {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let arr = vec![4, 3, 2, 1, 0];
    let res = 1;
    assert_eq!(Solution::max_chunks_to_sorted(arr), res);
    let arr = vec![1, 0, 2, 3, 4];
    let res = 4;
    assert_eq!(Solution::max_chunks_to_sorted(arr), res);
}
