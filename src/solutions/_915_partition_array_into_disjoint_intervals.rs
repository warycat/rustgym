struct Solution;

impl Solution {
    fn partition_disjoint(a: Vec<i32>) -> i32 {
        let n = a.len();
        let mut left = vec![0; n];
        let mut right = vec![0; n];
        let mut min = std::i32::MAX;
        let mut max = std::i32::MIN;
        for i in (0..n).rev() {
            min = min.min(a[i]);
            right[i] = min;
        }

        for i in 0..n {
            max = max.max(a[i]);
            left[i] = max;
        }
        for i in 1..n {
            if right[i] >= left[i - 1] {
                return i as i32;
            }
        }
        0
    }
}

#[test]
fn test() {
    let a = vec![5, 0, 3, 8, 6];
    let res = 3;
    assert_eq!(Solution::partition_disjoint(a), res);
    let a = vec![1, 1, 1, 0, 6, 12];
    let res = 4;
    assert_eq!(Solution::partition_disjoint(a), res);
}
