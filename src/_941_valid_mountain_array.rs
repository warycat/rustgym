struct Solution;

impl Solution {
    fn valid_mountain_array(a: Vec<i32>) -> bool {
        let n = a.len();
        if n < 3 {
            return false;
        }
        let mut i = 0;
        while i + 1 < n && a[i] < a[i + 1] {
            i += 1;
        }
        if i == 0 || i == n - 1 {
            return false;
        }
        while i + 1 < n && a[i] > a[i + 1] {
            i += 1;
        }
        i == n - 1
    }
}

#[test]
fn test() {
    let a = vec![2, 1];
    assert_eq!(Solution::valid_mountain_array(a), false);
    let a = vec![3, 5, 5];
    assert_eq!(Solution::valid_mountain_array(a), false);
    let a = vec![0, 3, 2, 1];
    assert_eq!(Solution::valid_mountain_array(a), true);
}
