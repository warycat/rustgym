struct Solution;

impl Solution {
    fn can_be_equal(mut target: Vec<i32>, mut arr: Vec<i32>) -> bool {
        target.sort_unstable();
        arr.sort_unstable();
        target == arr
    }
}

#[test]
fn test() {
    let target = vec![1, 2, 3, 4];
    let arr = vec![2, 4, 1, 3];
    let res = true;
    assert_eq!(Solution::can_be_equal(target, arr), res);
    let target = vec![3, 7, 9];
    let arr = vec![3, 7, 11];
    let res = false;
    assert_eq!(Solution::can_be_equal(target, arr), res);
}
