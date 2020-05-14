struct Solution;

impl Solution {
    fn max_subarray_sum_circular(a: Vec<i32>) -> i32 {
        let n = a.len();
        let sum = a.iter().sum::<i32>();
        let mut prev_min = 0;
        let mut prev_max = 0;
        let mut min = std::i32::MAX;
        let mut max = std::i32::MIN;
        for i in 0..n {
            prev_min = a[i].min(prev_min + a[i]);
            min = min.min(prev_min);
            prev_max = a[i].max(prev_max + a[i]);
            max = max.max(prev_max);
        }
        if max < 0 {
            max
        } else {
            max.max(sum - min)
        }
    }
}

#[test]
fn test() {
    let a = vec![1, -2, 3, -2];
    let res = 3;
    assert_eq!(Solution::max_subarray_sum_circular(a), res);
    let a = vec![5, -3, 5];
    let res = 10;
    assert_eq!(Solution::max_subarray_sum_circular(a), res);
    let a = vec![3, -1, 2, -1];
    let res = 4;
    assert_eq!(Solution::max_subarray_sum_circular(a), res);
    let a = vec![3, -2, 2, -3];
    let res = 3;
    assert_eq!(Solution::max_subarray_sum_circular(a), res);
    let a = vec![-2, -3, -1];
    let res = -1;
    assert_eq!(Solution::max_subarray_sum_circular(a), res);
}
