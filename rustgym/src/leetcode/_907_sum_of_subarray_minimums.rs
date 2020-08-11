struct Solution;

impl Solution {
    fn sum_subarray_mins(a: Vec<i32>) -> i32 {
        let n = a.len();
        let mut sum = 0;
        let mut left: Vec<usize> = vec![0; n];
        let mut right: Vec<usize> = vec![0; n];
        let mut stack: Vec<usize> = vec![];
        for i in 0..n {
            left[i] = i + 1;
            right[i] = n - i;
            while let Some(j) = stack.pop() {
                if a[j] < a[i] {
                    stack.push(j);
                    break;
                } else {
                    right[j] = i - j;
                }
            }
            if let Some(&j) = stack.last() {
                left[i] = i - j;
            }
            stack.push(i);
        }
        for i in 0..n {
            sum += (left[i] * right[i]) as i32 * a[i];
            sum %= 1_000_000_007;
        }
        sum
    }
}

#[test]
fn test() {
    let a = vec![3, 1, 2, 4];
    let res = 17;
    assert_eq!(Solution::sum_subarray_mins(a), res);
}
