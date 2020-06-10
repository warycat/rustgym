struct Solution;

impl Solution {
    fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut x: i32 = 0;
        for i in 0..k {
            x *= 10;
            x += 1;
            x %= k;
            if x % k == 0 {
                return i + 1;
            }
        }
        -1
    }
}

#[test]
fn test() {
    let k = 1;
    let res = 1;
    assert_eq!(Solution::smallest_repunit_div_by_k(k), res);
    let k = 2;
    let res = -1;
    assert_eq!(Solution::smallest_repunit_div_by_k(k), res);
    let k = 3;
    let res = 3;
    assert_eq!(Solution::smallest_repunit_div_by_k(k), res);
}
