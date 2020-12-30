struct Solution;

impl Solution {
    fn maximum_sum(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut left_sum = 0;
        let mut left_max = vec![0; n];
        let mut left_min = 0;
        let mut right_sum = 0;
        let mut right_max = vec![0; n];
        let mut right_min = 0;
        for i in 0..n {
            left_sum += arr[i];
            left_max[i] = left_sum - left_min;
            left_min = left_min.min(left_sum);
        }
        for i in (0..n).rev() {
            right_sum += arr[i];
            right_max[i] = right_sum - right_min;
            right_min = right_min.min(right_sum);
        }
        let mut res = std::i32::MIN;
        for i in 0..n {
            res = res.max(left_max[i]);
            res = res.max(right_max[i]);
            if i > 0 && i + 1 < n {
                res = res.max(left_max[i - 1] + right_max[i + 1]);
            }
        }
        res
    }
}

#[test]
fn test() {
    let arr = vec![1, -2, 0, 3];
    let res = 4;
    assert_eq!(Solution::maximum_sum(arr), res);
    let arr = vec![1, -2, -2, 3];
    let res = 3;
    assert_eq!(Solution::maximum_sum(arr), res);
    let arr = vec![-1, -1, -1, -1];
    let res = -1;
    assert_eq!(Solution::maximum_sum(arr), res);
    let arr = vec![0, -5, -6, 5, 0, -5];
    let res = 5;
    assert_eq!(Solution::maximum_sum(arr), res);
}
