struct Solution;

impl Solution {
    fn last_remaining(n: i32) -> i32 {
        if n == 1 {
            1
        } else {
            2 * (1 + n / 2 - Self::last_remaining(n / 2))
        }
    }
}

#[test]
fn test() {
    let n = 9;
    let res = 6;
    assert_eq!(Solution::last_remaining(n), res);
}
