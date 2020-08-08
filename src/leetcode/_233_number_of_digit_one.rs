struct Solution;

impl Solution {
    fn count_digit_one(n: i32) -> i32 {
        let mut m = 1i64;
        let mut res = 0i64;
        let n = n as i64;
        while m <= n {
            let d = 10 * m;
            res += n / d * m + m.min(0.max(n % d - m + 1));
            m *= 10;
        }
        res as i32
    }
}

#[test]
fn test() {
    let n = 13;
    let res = 6;
    assert_eq!(Solution::count_digit_one(n), res);
}
