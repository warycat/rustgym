struct Solution;

impl Solution {
    fn range_bitwise_and(mut m: i32, mut n: i32) -> i32 {
        let mut shift = 0;
        while m != n {
            m >>= 1;
            n >>= 1;
            shift += 1;
        }
        n << shift
    }
}

#[test]
fn test() {
    let m = 5;
    let n = 7;
    let res = 4;
    assert_eq!(Solution::range_bitwise_and(m, n), res);
    let m = 0;
    let n = 1;
    let res = 0;
    assert_eq!(Solution::range_bitwise_and(m, n), res);
}
