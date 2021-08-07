struct Solution;

impl Solution {
    fn min_swaps(data: Vec<i32>) -> i32 {
        let n = data.len();
        let m = data.iter().sum::<i32>() as usize;
        let mut sum = 0;
        let mut max_sum = 0;
        for i in 0..n {
            sum += data[i];
            if i >= m {
                sum -= data[i - m];
            }
            max_sum = max_sum.max(sum);
        }

        m as i32 - max_sum
    }
}

#[test]
fn test() {
    let data = vec![1, 0, 1, 0, 1];
    let res = 1;
    assert_eq!(Solution::min_swaps(data), res);
    let data = vec![0, 0, 0, 1, 0];
    let res = 0;
    assert_eq!(Solution::min_swaps(data), res);
    let data = vec![1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1];
    let res = 3;
    assert_eq!(Solution::min_swaps(data), res);
}
