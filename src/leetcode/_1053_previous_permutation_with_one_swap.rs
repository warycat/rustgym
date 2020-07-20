struct Solution;

impl Solution {
    fn prev_perm_opt1(mut a: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        if n < 2 {
            return a;
        }
        let mut i = n - 2;
        while i > 0 && a[i] <= a[i + 1] {
            i -= 1;
        }
        if i == 0 && a[0] <= a[1] {
            return a;
        }
        let mut j = n - 1;
        while a[j] >= a[i] || a[j] == a[j - 1] {
            j -= 1;
        }
        a.swap(i, j);
        a
    }
}

#[test]
fn test() {
    let a = vec![3, 2, 1];
    let res = vec![3, 1, 2];
    assert_eq!(Solution::prev_perm_opt1(a), res);
    let a = vec![1, 1, 5];
    let res = vec![1, 1, 5];
    assert_eq!(Solution::prev_perm_opt1(a), res);
    let a = vec![1, 9, 4, 6, 7];
    let res = vec![1, 7, 4, 6, 9];
    assert_eq!(Solution::prev_perm_opt1(a), res);
    let a = vec![3, 1, 1, 3];
    let res = vec![1, 3, 1, 3];
    assert_eq!(Solution::prev_perm_opt1(a), res);
}
