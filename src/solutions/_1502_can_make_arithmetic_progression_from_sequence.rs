struct Solution;

impl Solution {
    fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort_unstable();
        let diff: Vec<i32> = arr.windows(2).map(|v| v[1] - v[0]).collect();
        diff.iter().min() == diff.iter().max()
    }
}

#[test]
fn test() {
    let arr = vec![3, 5, 1];
    let res = true;
    assert_eq!(Solution::can_make_arithmetic_progression(arr), res);
    let arr = vec![1, 2, 4];
    let res = false;
    assert_eq!(Solution::can_make_arithmetic_progression(arr), res);
}
