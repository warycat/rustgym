struct Solution;

impl Solution {
    fn divisor_game(n: i32) -> bool {
        n % 2 == 0
    }
}

#[test]
fn test() {
    assert_eq!(Solution::divisor_game(2), true);
    assert_eq!(Solution::divisor_game(3), false);
}
