struct Solution;

impl Solution {
    fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        left.into_iter()
            .chain(right.into_iter().map(|x| n - x))
            .max()
            .unwrap()
    }
}

#[test]
fn test() {
    let n = 4;
    let left = vec![4, 3];
    let right = vec![0, 1];
    let res = 4;
    assert_eq!(Solution::get_last_moment(n, left, right), res);
    let n = 7;
    let left = vec![];
    let right = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let res = 7;
    assert_eq!(Solution::get_last_moment(n, left, right), res);
    let n = 9;
    let left = vec![5];
    let right = vec![4];
    let res = 5;
    assert_eq!(Solution::get_last_moment(n, left, right), res);
    let n = 6;
    let left = vec![6];
    let right = vec![0];
    let res = 6;
    assert_eq!(Solution::get_last_moment(n, left, right), res);
}
