struct Solution;
use std::collections::HashMap;

impl Solution {
    fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let n = arr.len();
        let mut hm: HashMap<i32, i32> = HashMap::new();
        let mut dp: Vec<i32> = vec![std::i32::MAX; n];
        *hm.entry(0).or_default() = -1;
        let mut prev = 0;
        let mut res = std::i32::MAX;
        let mut min = std::i32::MAX;
        for i in 0..n {
            prev += arr[i];
            if let Some(&j) = hm.get(&(prev - target)) {
                if j > -1 && dp[j as usize] != std::i32::MAX {
                    res = res.min(i as i32 - j + dp[j as usize]);
                }
                min = min.min(i as i32 - j);
            }
            dp[i] = min;
            *hm.entry(prev).or_default() = i as i32;
        }

        if res == std::i32::MAX {
            -1
        } else {
            res
        }
    }
}

#[test]
fn test() {
    let arr = vec![3, 2, 2, 4, 3];
    let target = 3;
    let res = 2;
    assert_eq!(Solution::min_sum_of_lengths(arr, target), res);
    let arr = vec![7, 3, 4, 7];
    let target = 7;
    let res = 2;
    assert_eq!(Solution::min_sum_of_lengths(arr, target), res);
    let arr = vec![4, 3, 2, 6, 2, 3, 4];
    let target = 6;
    let res = -1;
    assert_eq!(Solution::min_sum_of_lengths(arr, target), res);
    let arr = vec![5, 5, 4, 4, 5];
    let target = 3;
    let res = -1;
    assert_eq!(Solution::min_sum_of_lengths(arr, target), res);
    let arr = vec![3, 1, 1, 1, 5, 1, 2, 1];
    let target = 3;
    let res = 3;
    assert_eq!(Solution::min_sum_of_lengths(arr, target), res);
    let arr = vec![1, 6, 1];
    let target = 7;
    let res = -1;
    assert_eq!(Solution::min_sum_of_lengths(arr, target), res);
}
