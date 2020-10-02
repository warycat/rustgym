struct Solution;

impl Solution {
    fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let n = arr.len() as i32;
        let mut res = 0;
        for i in 0..n {
            res += ((i + 1) * (n - i) + 1) / 2 * arr[i as usize];
        }
        res
    }
}

#[test]
fn test() {
    let arr = vec![1, 4, 2, 5, 3];
    let res = 58;
    assert_eq!(Solution::sum_odd_length_subarrays(arr), res);
    let arr = vec![1, 2];
    let res = 3;
    assert_eq!(Solution::sum_odd_length_subarrays(arr), res);
    let arr = vec![10, 11, 12];
    let res = 66;
    assert_eq!(Solution::sum_odd_length_subarrays(arr), res);
}
