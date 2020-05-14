struct Solution;

const M: i32 = 1_000_000_007;

impl Solution {
    fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; 5001]; 16]; 6];
        Self::dfs(n, -1, 1, &mut dp, &roll_max)
    }

    fn dfs(
        n: usize,
        prev: i32,
        repeat: usize,
        dp: &mut Vec<Vec<Vec<i32>>>,
        roll_max: &[i32],
    ) -> i32 {
        if n == 0 {
            1
        } else {
            if prev >= 0 && dp[prev as usize][repeat][n] > 0 {
                return dp[prev as usize][repeat][n];
            }
            let mut sum = 0;
            for i in 0..6 {
                if i == prev {
                    if repeat < roll_max[i as usize] as usize {
                        sum += Self::dfs(n - 1, i, repeat + 1, dp, roll_max);
                        sum %= M;
                    }
                } else {
                    sum += Self::dfs(n - 1, i, 1, dp, roll_max);
                    sum %= M;
                }
            }
            if prev >= 0 {
                dp[prev as usize][repeat][n] = sum;
            }
            sum
        }
    }
}

#[test]
fn test() {
    let n = 2;
    let roll_max = vec![1, 1, 2, 2, 2, 3];
    let res = 34;
    assert_eq!(Solution::die_simulator(n, roll_max), res);
    let n = 2;
    let roll_max = vec![1, 1, 1, 1, 1, 1];
    let res = 30;
    assert_eq!(Solution::die_simulator(n, roll_max), res);
    let n = 3;
    let roll_max = vec![1, 1, 1, 2, 2, 3];
    let res = 181;
    assert_eq!(Solution::die_simulator(n, roll_max), res);
    let n = 4;
    let roll_max = vec![2, 1, 1, 3, 3, 2];
    let res = 1082;
    assert_eq!(Solution::die_simulator(n, roll_max), res);
}
