struct Solution;

impl Solution {
    fn super_pow(a: i32, mut b: Vec<i32>) -> i32 {
        let a = a % 1337;
        if let Some(last) = b.pop() {
            Self::pow_mod(Self::super_pow(a, b) % 1337, 10) * Self::pow_mod(a, last) % 1337
        } else {
            1
        }
    }

    fn pow_mod(a: i32, k: i32) -> i32 {
        let mut res = 1;
        for _ in 0..k {
            res *= a;
            res %= 1337;
        }
        res
    }
}

#[test]
fn test() {
    let a = 2;
    let b = vec![3];
    let res = 8;
    assert_eq!(Solution::super_pow(a, b), res);
    let a = 2;
    let b = vec![1, 0];
    let res = 1024;
    assert_eq!(Solution::super_pow(a, b), res);
}
