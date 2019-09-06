struct Solution;

impl Solution {
    fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let mut mask = !0;
        while mask & n != 0 {
            mask <<= 1;
        }
        mask = !mask;
        mask ^ n
    }
}

#[test]
fn test() {
    assert_eq!(Solution::bitwise_complement(5), 2);
    assert_eq!(Solution::bitwise_complement(7), 0);
    assert_eq!(Solution::bitwise_complement(10), 5);
    assert_eq!(Solution::bitwise_complement(0), 1);
}
