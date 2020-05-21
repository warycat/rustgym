struct Solution;

impl Solution {
    fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}

#[test]
fn test() {
    assert_eq!(Solution::can_win_nim(4), false);
}
