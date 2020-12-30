struct Solution;

impl Solution {
    fn minimum_moves(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut memo = vec![vec![0; n + 1]; n];
        Self::dp(0, n, &mut memo, &arr, n)
    }
    fn dp(start: usize, size: usize, memo: &mut Vec<Vec<i32>>, arr: &[i32], n: usize) -> i32 {
        if size <= 1 {
            1
        } else {
            if memo[start][size] != 0 {
                memo[start][size]
            } else {
                let mut res = std::i32::MAX;
                if arr[start] == arr[start + size - 1] {
                    res = res.min(Self::dp(start + 1, size - 2, memo, arr, n));
                }
                for lsize in 1..size {
                    let rsize = size - lsize;
                    let l = Self::dp(start, lsize, memo, arr, n);
                    let r = Self::dp(start + lsize, rsize, memo, arr, n);
                    res = res.min(l + r);
                }
                memo[start][size] = res;
                res
            }
        }
    }
}

#[test]
fn test() {
    let arr = vec![1, 2];
    let res = 2;
    assert_eq!(Solution::minimum_moves(arr), res);
    let arr = vec![1, 3, 4, 1, 5];
    let res = 3;
    assert_eq!(Solution::minimum_moves(arr), res);
    let arr = vec![16, 13, 13, 10, 12];
    let res = 4;
    assert_eq!(Solution::minimum_moves(arr), res);
    let arr = vec![1, 4, 1, 1, 2, 3, 2, 1];
    let res = 2;
    assert_eq!(Solution::minimum_moves(arr), res);
}
