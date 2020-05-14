struct Solution;

impl Solution {
    fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
        let mut x = 0;
        let n = a.len();
        let mut res: Vec<bool> = vec![false; n];
        for i in 0..n {
            x = (x * 2 + a[i]) % 5;
            res[i] = x == 0;
        }
        res
    }
}

#[test]
fn test() {
    let a = vec![0, 1, 1];
    let res = vec![true, false, false];
    assert_eq!(Solution::prefixes_div_by5(a), res);
    let a = vec![1, 1, 1];
    let res = vec![false, false, false];
    assert_eq!(Solution::prefixes_div_by5(a), res);
    let a = vec![0, 1, 1, 1, 1, 1];
    let res = vec![true, false, false, false, true, false];
    assert_eq!(Solution::prefixes_div_by5(a), res);
    let a = vec![1, 1, 1, 0, 1];
    let res = vec![false, false, false, false, false];
    assert_eq!(Solution::prefixes_div_by5(a), res);
}
