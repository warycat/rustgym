struct Solution;
use std::collections::HashSet;

impl Solution {
    fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut hs: HashSet<i32> = HashSet::new();
        let mut zero = 0;
        for &x in &arr {
            if x == 0 {
                if zero > 0 {
                    return true;
                } else {
                    zero += 1;
                }
            } else {
                hs.insert(x);
            }
        }
        for x in arr {
            if hs.contains(&(2 * x)) {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    let arr = vec![10, 2, 5, 3];
    let res = true;
    assert_eq!(Solution::check_if_exist(arr), res);
    let arr = vec![7, 1, 14, 11];
    let res = true;
    assert_eq!(Solution::check_if_exist(arr), res);
    let arr = vec![3, 1, 7, 11];
    let res = false;
    assert_eq!(Solution::check_if_exist(arr), res);
    let arr = vec![-2, 0, 10, -19, 4, 6, -8];
    let res = false;
    assert_eq!(Solution::check_if_exist(arr), res);
}
