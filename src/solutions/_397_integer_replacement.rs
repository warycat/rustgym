struct Solution;

impl Solution {
    fn integer_replacement(n: i32) -> i32 {
        Self::dfs(n as u32) as i32
    }
    fn dfs(mut n: u32) -> usize {
        if n == 1 {
            0
        } else {
            let mut zeros = 0;
            while n & 1 == 0 {
                n >>= 1;
                zeros += 1;
            }
            if n == 1 {
                zeros
            } else {
                zeros + 1 + Self::dfs(n + 1).min(Self::dfs(n - 1))
            }
        }
    }
}

#[test]
fn test() {
    let n = 8;
    let res = 3;
    assert_eq!(Solution::integer_replacement(n), res);
    let n = 7;
    let res = 4;
    assert_eq!(Solution::integer_replacement(n), res);
    let n = 2147483647;
    let res = 32;
    assert_eq!(Solution::integer_replacement(n), res);
}
