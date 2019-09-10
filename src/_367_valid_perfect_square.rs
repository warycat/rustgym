struct Solution;

impl Solution {
    fn is_perfect_square(num: i32) -> bool {
        let mut r: i64 = num as i64;
        let x: i64 = num as i64;
        while r * r > x {
            r = (r + x / r) / 2;
        }
        r * r == x
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_perfect_square(16), true);
    assert_eq!(Solution::is_perfect_square(14), false);
}
