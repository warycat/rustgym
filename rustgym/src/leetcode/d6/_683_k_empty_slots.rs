struct Solution;

use std::collections::BTreeSet;

impl Solution {
    fn k_empty_slots(bulbs: Vec<i32>, k: i32) -> i32 {
        let n = bulbs.len();
        let mut set: BTreeSet<i32> = BTreeSet::new();
        for i in 0..n {
            let day = (i + 1) as i32;
            let mid = bulbs[i];
            if let Some(left) = set.range(..mid).next_back() {
                if mid - left - 1 == k {
                    return day;
                }
            }
            if let Some(right) = set.range(mid..).next() {
                if right - mid - 1 == k {
                    return day;
                }
            }
            set.insert(mid);
        }
        -1
    }
}

#[test]
fn test() {
    let bulbs = vec![1, 3, 2];
    let k = 1;
    let res = 2;
    assert_eq!(Solution::k_empty_slots(bulbs, k), res);
    let bulbs = vec![1, 3, 2];
    let k = 1;
    let res = 2;
    assert_eq!(Solution::k_empty_slots(bulbs, k), res);
}
