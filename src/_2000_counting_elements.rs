struct Solution;
use std::collections::HashSet;

impl Solution {
    fn count_elements(arr: Vec<i32>) -> i32 {
        let mut hs: HashSet<i32> = HashSet::new();
        for &x in &arr {
            hs.insert(x);
        }
        let mut res = 0;
        for &x in &arr {
            if hs.contains(&(x + 1)) {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let arr = vec![1, 2, 3];
    let res = 2;
    assert_eq!(Solution::count_elements(arr), res);
    let arr = vec![1, 1, 3, 3, 5, 5, 7, 7];
    let res = 0;
    assert_eq!(Solution::count_elements(arr), res);
    let arr = vec![1, 3, 2, 3, 5, 0];
    let res = 3;
    assert_eq!(Solution::count_elements(arr), res);
    let arr = vec![1, 1, 2, 2];
    let res = 2;
    assert_eq!(Solution::count_elements(arr), res);
}
