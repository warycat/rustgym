struct Solution;

impl Solution {
    fn new_integer(mut n: i32) -> i32 {
        let mut res = 0;
        let mut base = 1;
        while n > 0 {
            res += n % 9 * base;
            n /= 9;
            base *= 10;
        }
        res
    }
}

#[test]
fn test() {
    let n = 9;
    let res = 10;
    assert_eq!(Solution::new_integer(n), res);
}
