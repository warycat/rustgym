struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn num_sub(s: String) -> i32 {
        let mut res: i64 = 0;
        let mut it = s.chars().peekable();
        while let Some(c) = it.next() {
            if c == '1' {
                let mut n = 1;
                while let Some('1') = it.peek() {
                    it.next();
                    n += 1;
                }
                res += n * (n + 1) / 2;
                res %= MOD;
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let s = "0110111".to_string();
    let res = 9;
    assert_eq!(Solution::num_sub(s), res);
    let s = "101".to_string();
    let res = 2;
    assert_eq!(Solution::num_sub(s), res);
    let s = "111111".to_string();
    let res = 21;
    assert_eq!(Solution::num_sub(s), res);
    let s = "000".to_string();
    let res = 0;
    assert_eq!(Solution::num_sub(s), res);
}
