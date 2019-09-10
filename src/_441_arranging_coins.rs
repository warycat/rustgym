struct Solution;

impl Solution {
    fn arrange_coins(n: i32) -> i32 {
        (((2 * n as i64) as f64 + 0.25).sqrt() - 0.5).floor() as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::arrange_coins(5), 2);
    assert_eq!(Solution::arrange_coins(8), 3);
    assert_eq!(Solution::arrange_coins(1_804_289_383), 60070);
}
