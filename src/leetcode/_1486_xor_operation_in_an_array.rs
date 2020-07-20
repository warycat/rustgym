struct Solution;

impl Solution {
    fn xor_operation(n: i32, start: i32) -> i32 {
        (0..n).fold(0, |acc, i| acc ^ (start + 2 * i))
    }
}

#[test]
fn test() {
    let n = 5;
    let start = 0;
    let res = 8;
    assert_eq!(Solution::xor_operation(n, start), res);
    let n = 4;
    let start = 3;
    let res = 8;
    assert_eq!(Solution::xor_operation(n, start), res);
    let n = 1;
    let start = 7;
    let res = 7;
    assert_eq!(Solution::xor_operation(n, start), res);
    let n = 10;
    let start = 5;
    let res = 2;
    assert_eq!(Solution::xor_operation(n, start), res);
}
