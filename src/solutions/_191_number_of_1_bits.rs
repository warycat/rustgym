#![allow(clippy::unreadable_literal)]
struct Solution;

impl Solution {
    #[allow(non_snake_case)]
    fn hammingWeight(n: u32) -> i32 {
        n.count_ones() as i32
    }
}

#[test]
fn test() {
    let n = 0b00000000000000000000000000001011;
    let res = 3;
    assert_eq!(Solution::hammingWeight(n), res);
    let n = 0b00000000000000000000000010000000;
    let res = 1;
    assert_eq!(Solution::hammingWeight(n), res);
    let n = 0b11111111111111111111111111111101;
    let res = 31;
    assert_eq!(Solution::hammingWeight(n), res);
}
