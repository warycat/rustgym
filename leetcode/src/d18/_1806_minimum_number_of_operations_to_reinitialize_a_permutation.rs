struct Solution;

impl Solution {
    fn reinitialize_permutation(n: i32) -> i32 {
        if n == 2 {
            return 1;
        }
        let mut res = 0;
        let mut i = 1;
        while res == 0 || i != 1 {
            if i % 2 == 0 {
                i /= 2;
            } else {
                i = (n - 1 + i) / 2;
            }
            res += 1;
        }
        res
    }
}

#[test]
fn test() {
    let n = 2;
    let res = 1;
    assert_eq!(Solution::reinitialize_permutation(n), res);
    let n = 4;
    let res = 2;
    assert_eq!(Solution::reinitialize_permutation(n), res);
    let n = 6;
    let res = 4;
    assert_eq!(Solution::reinitialize_permutation(n), res);
}
