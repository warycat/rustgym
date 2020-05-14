struct Solution;

impl Solution {
    fn is_ugly(mut num: i32) -> bool {
        if num < 1 {
            return false;
        }
        while num % 2 == 0 {
            num /= 2;
        }
        while num % 3 == 0 {
            num /= 3;
        }
        while num % 5 == 0 {
            num /= 5;
        }
        num == 1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_ugly(6), true);
    assert_eq!(Solution::is_ugly(8), true);
    assert_eq!(Solution::is_ugly(14), false);
}
