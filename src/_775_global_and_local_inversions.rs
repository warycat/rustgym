struct Solution;

impl Solution {
    fn is_ideal_permutation(a: Vec<i32>) -> bool {
        let n = a.len();
        for i in 0..n {
            if (a[i] - i as i32).abs() > 1 {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let a = vec![1, 0, 2];
    let res = true;
    assert_eq!(Solution::is_ideal_permutation(a), res);
    let a = vec![1, 2, 0];
    let res = false;
    assert_eq!(Solution::is_ideal_permutation(a), res);
}
