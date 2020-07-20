struct Solution;

impl Solution {
    fn fixed_point(a: Vec<i32>) -> i32 {
        for i in 0..a.len() {
            if i as i32 == a[i] {
                return i as i32;
            }
        }
        -1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::fixed_point(vec![-10, -5, 0, 3, 7]), 3);
    assert_eq!(Solution::fixed_point(vec![0, 2, 5, 8, 17]), 0);
    assert_eq!(Solution::fixed_point(vec![-10, -5, 3, 4, 7, 9]), -1);
}
