struct Solution;

impl Solution {
    fn pivot_index(num: Vec<i32>) -> i32 {
        let n = num.len();
        if n == 0 {
            return -1;
        }
        let mut presum = vec![0; n];
        for i in 0..n {
            if i == 0 {
                presum[0] = num[0]
            } else {
                presum[i] = presum[i - 1] + num[i];
            }
        }
        for i in 0..n {
            let mut left = 0;
            let total = presum[n - 1];
            let middle = num[i];
            if i > 0 {
                left = presum[i - 1];
            }
            if left * 2 + middle == total {
                return i as i32;
            }
        }
        -1
    }
}

#[test]
fn exist() {
    assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
}

#[test]
fn no_index() {
    assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
}
