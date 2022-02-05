struct Solution;

impl Solution {
    fn is_same_after_reversals(num: i32) -> bool {
        num == 0 || num % 10 != 0
    }
}

#[test]
fn test() {
    let num = 526;
    let res = true;
    assert_eq!(Solution::is_same_after_reversals(num), res);
    let num = 1800;
    let res = false;
    assert_eq!(Solution::is_same_after_reversals(num), res);
    let num = 0;
    let res = true;
    assert_eq!(Solution::is_same_after_reversals(num), res);
}
