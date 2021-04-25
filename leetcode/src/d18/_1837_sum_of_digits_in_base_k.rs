struct Solution;

impl Solution {
    fn sum_base(mut n: i32, k: i32) -> i32 {
        let mut res = 0;
        while n > 0 {
            res += n % k;
            n /= k;
        }
        res
    }
}

#[test]
fn test() {
    let n = 34;
    let k = 6;
    let res = 9;
    assert_eq!(Solution::sum_base(n, k), res);
    let n = 10;
    let k = 10;
    let res = 1;
    assert_eq!(Solution::sum_base(n, k), res);
}
