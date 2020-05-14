struct Solution;

use std::collections::HashSet;

impl Solution {
    fn repeated_n_times(a: Vec<i32>) -> i32 {
        let mut hs: HashSet<i32> = HashSet::new();
        for x in a {
            if !hs.insert(x) {
                return x;
            }
        }
        unreachable!()
    }
}

#[test]
fn test() {
    let a = vec![1, 2, 3, 3];
    assert_eq!(Solution::repeated_n_times(a), 3);
    let a = vec![2, 1, 2, 5, 3, 2];
    assert_eq!(Solution::repeated_n_times(a), 2);
    let a = vec![5, 1, 5, 2, 5, 3, 5, 4];
    assert_eq!(Solution::repeated_n_times(a), 5);
}
