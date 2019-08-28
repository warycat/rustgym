struct Solution;

impl Solution {
    fn largest_perimeter(mut a: Vec<i32>) -> i32 {
        let n = a.len();
        a.sort_unstable();
        for i in (0..=n - 3).rev() {
            if a[i] + a[i + 1] > a[i + 2] {
                return a[i] + a[i + 1] + a[i + 2];
            }
        }
        0
    }
}

#[test]
fn test() {
    let a = vec![2, 1, 2];
    assert_eq!(Solution::largest_perimeter(a), 5);
    let a = vec![1, 2, 1];
    assert_eq!(Solution::largest_perimeter(a), 0);
    let a = vec![3, 2, 3, 4];
    assert_eq!(Solution::largest_perimeter(a), 10);
    let a = vec![3, 6, 2, 3];
    assert_eq!(Solution::largest_perimeter(a), 8);
}
