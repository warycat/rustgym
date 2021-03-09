struct Solution;

impl Solution {
    fn is_monotonic(a: Vec<i32>) -> bool {
        let mut increasing = true;
        let mut decreasing = true;
        let n = a.len();
        for i in 1..n {
            if a[i] > a[i - 1] {
                decreasing = false;
            }
            if a[i] < a[i - 1] {
                increasing = false;
            }
        }
        increasing || decreasing
    }
}

#[test]
fn test() {
    let a = vec![1, 2, 2, 3];
    assert_eq!(Solution::is_monotonic(a), true);
    let a = vec![6, 5, 4, 4];
    assert_eq!(Solution::is_monotonic(a), true);
    let a = vec![1, 3, 2];
    assert_eq!(Solution::is_monotonic(a), false);
    let a = vec![1, 2, 4, 5];
    assert_eq!(Solution::is_monotonic(a), true);
    let a = vec![1, 1, 1];
    assert_eq!(Solution::is_monotonic(a), true);
}
