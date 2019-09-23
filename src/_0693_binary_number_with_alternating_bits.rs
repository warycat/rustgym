struct Solution;

impl Solution {
    fn has_alternating_bits(n: i32) -> bool {
        let x = (n >> 1) ^ n;
        (x + 1) & x == 0
    }
}

#[test]
fn test() {
    assert_eq!(Solution::has_alternating_bits(5), true);
    assert_eq!(Solution::has_alternating_bits(7), false);
    assert_eq!(Solution::has_alternating_bits(11), false);
    assert_eq!(Solution::has_alternating_bits(1), true);
}
