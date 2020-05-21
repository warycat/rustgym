struct Solution;

impl Solution {
    fn knight_dialer(n: i32) -> i32 {
        let max = 1_000_000_007;
        let n = n as usize;
        let mut dp: [[usize; 10]; 2] = [
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        let map = vec![
            vec![4, 6],
            vec![6, 8],
            vec![7, 9],
            vec![4, 8],
            vec![0, 3, 9],
            vec![],
            vec![0, 1, 7],
            vec![2, 6],
            vec![1, 3],
            vec![2, 4],
        ];
        let mut res = 10;
        for i in 1..n {
            res = 0;
            for j in 0..10 {
                let mut sum = 0;
                for &k in &map[j] {
                    sum += dp[(i - 1) % 2][k];
                    sum %= max;
                }
                dp[i % 2][j] = sum;
                res += dp[i % 2][j];
                res %= max;
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let n = 1;
    let res = 10;
    assert_eq!(Solution::knight_dialer(n), res);
    let n = 2;
    let res = 20;
    assert_eq!(Solution::knight_dialer(n), res);
    let n = 3;
    let res = 46;
    assert_eq!(Solution::knight_dialer(n), res);
    let n = 161;
    let res = 533_302_150;
    assert_eq!(Solution::knight_dialer(n), res);
}
