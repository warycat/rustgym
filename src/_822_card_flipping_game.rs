struct Solution;
use std::collections::HashSet;

impl Solution {
    fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
        let n = fronts.len();
        let mut hs: HashSet<i32> = HashSet::new();
        let mut res = std::i32::MAX;
        for i in 0..n {
            if fronts[i] == backs[i] {
                hs.insert(fronts[i]);
            }
        }
        for i in 0..n {
            if !hs.contains(&fronts[i]) {
                res = res.min(fronts[i]);
            }
            if !hs.contains(&backs[i]) {
                res = res.min(backs[i]);
            }
        }
        if res == std::i32::MAX {
            0
        } else {
            res
        }
    }
}

#[test]
fn test() {
    let fronts = vec![1, 2, 4, 4, 7];
    let backs = vec![1, 3, 4, 1, 3];
    let res = 2;
    assert_eq!(Solution::flipgame(fronts, backs), res);
}
