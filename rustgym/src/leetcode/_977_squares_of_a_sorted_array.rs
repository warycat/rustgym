struct Solution;

impl Solution {
    fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut squared: Vec<i32> = a.iter().map(|a| a * a).collect();
        squared.sort_unstable();
        squared
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
        vec![0, 1, 9, 16, 100]
    );
    assert_eq!(
        Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
        vec![4, 9, 9, 49, 121]
    );
}
