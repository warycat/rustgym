struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn concatenated_binary(n: i32) -> i32 {
        let mut res = 0;
        let mut stack: Vec<i64> = vec![];
        for i in 1..=n {
            let mut j = i;
            while j > 0 {
                stack.push((j % 2) as i64);
                j >>= 1;
            }
            while let Some(top) = stack.pop() {
                res <<= 1;
                res += top;
            }
            res %= MOD;
        }
        res as i32
    }
}

#[test]
fn test() {
    let n = 1;
    let res = 1;
    assert_eq!(Solution::concatenated_binary(n), res);
    let n = 3;
    let res = 27;
    assert_eq!(Solution::concatenated_binary(n), res);
    let n = 12;
    let res = 505379714;
    assert_eq!(Solution::concatenated_binary(n), res);
}
