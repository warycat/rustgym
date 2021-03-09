struct Solution;

impl Solution {
    fn number_of_matches(mut n: i32) -> i32 {
        let mut res = 0;
        while n > 1 {
            if n % 2 == 1 {
                res += 1;
            }
            n /= 2;
            res += n;
        }
        res
    }
}

#[test]
fn test() {
    let n = 7;
    let res = 6;
    assert_eq!(Solution::number_of_matches(n), res);
    let n = 14;
    let res = 13;
    assert_eq!(Solution::number_of_matches(n), res);
}
