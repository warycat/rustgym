struct Solution;

impl Solution {
    fn consecutive_numbers_sum(n: i32) -> i32 {
        let mut i = 2;
        let mut res = 1;
        while i * i < 2 * n {
            if (n - i * (i - 1) / 2) % i == 0 {
                res += 1;
            }
            i += 1;
        }
        res
    }
}

#[test]
fn test() {
    let n = 5;
    let res = 2;
    assert_eq!(Solution::consecutive_numbers_sum(n), res);
    let n = 9;
    let res = 3;
    assert_eq!(Solution::consecutive_numbers_sum(n), res);
    let n = 15;
    let res = 4;
    assert_eq!(Solution::consecutive_numbers_sum(n), res);
    let n = 4;
    let res = 1;
    assert_eq!(Solution::consecutive_numbers_sum(n), res);
}
