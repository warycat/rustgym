struct Solution;

impl Solution {
    fn max_rotate_function(a: Vec<i32>) -> i32 {
        let n = a.len();
        if n == 0 {
            return 0;
        }
        let sum: i32 = a.iter().sum();
        let mut f = 0;
        for i in 0..n {
            f += i as i32 * a[i];
        }
        let mut res = f;
        for i in (1..n).rev() {
            f = f + sum - n as i32 * a[i];
            res = res.max(f);
        }
        res
    }
}

#[test]
fn test() {
    let a = vec![4, 3, 2, 6];
    let res = 26;
    assert_eq!(Solution::max_rotate_function(a), res);
}
