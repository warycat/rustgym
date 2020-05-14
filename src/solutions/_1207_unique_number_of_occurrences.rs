struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        for x in arr {
            *hm.entry(x).or_default() += 1;
        }
        let mut hs = HashSet::new();
        for x in hm.values() {
            if !hs.insert(x) {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let arr = vec![1, 2, 2, 1, 1, 3];
    assert_eq!(Solution::unique_occurrences(arr), true);
    let arr = vec![1, 2];
    assert_eq!(Solution::unique_occurrences(arr), false);
    let arr = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
    assert_eq!(Solution::unique_occurrences(arr), true);
}
