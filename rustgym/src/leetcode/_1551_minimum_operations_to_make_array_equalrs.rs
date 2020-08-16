struct Solution;

impl Solution {
    fn min_operations(n: i32) -> i32 {
        let mut res = 0;
        let mut i = 1;
        while i < n {
            res += n - i;
            i += 2;
        }
        res
    }
}

#[test]
fn test() {
    let n = 3;
    let res = 2;
    assert_eq!(Solution::min_operations(n), res);
    let n = 6;
    let res = 9;
    assert_eq!(Solution::min_operations(n), res);
}
