struct Solution;

impl Solution {
    fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let n = arr.len();
        let mut prefix = vec![0; n + 1];
        let sum = threshold * k;
        let k = k as usize;
        let mut res = 0;
        for i in 0..n {
            prefix[i + 1] = prefix[i] + arr[i];
        }
        for r in k..=n {
            let l = r - k;
            if prefix[r] - prefix[l] >= sum {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let arr = vec![2, 2, 2, 2, 5, 5, 5, 8];
    let k = 3;
    let threshold = 4;
    let res = 3;
    assert_eq!(Solution::num_of_subarrays(arr, k, threshold), res);
    let arr = vec![1, 1, 1, 1, 1];
    let k = 1;
    let threshold = 0;
    let res = 5;
    assert_eq!(Solution::num_of_subarrays(arr, k, threshold), res);
    let arr = vec![11, 13, 17, 23, 29, 31, 7, 5, 2, 3];
    let k = 3;
    let threshold = 5;
    let res = 6;
    assert_eq!(Solution::num_of_subarrays(arr, k, threshold), res);
    let arr = vec![7, 7, 7, 7, 7, 7, 7];
    let k = 7;
    let threshold = 7;
    let res = 1;
    assert_eq!(Solution::num_of_subarrays(arr, k, threshold), res);
    let arr = vec![4, 4, 4, 4];
    let k = 4;
    let threshold = 1;
    let res = 1;
    assert_eq!(Solution::num_of_subarrays(arr, k, threshold), res);
}
