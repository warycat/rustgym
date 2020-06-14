struct Solution;
use std::collections::HashSet;

impl Solution {
    fn reordered_power_of2(n: i32) -> bool {
        let mut hs: HashSet<Vec<char>> = HashSet::new();
        for i in 0..32 {
            let mut v: Vec<char> = (1 << i).to_string().chars().collect();
            v.sort_unstable();
            hs.insert(v);
        }
        let mut v: Vec<char> = n.to_string().chars().collect();
        v.sort_unstable();
        hs.contains(&v)
    }
}

#[test]
fn test() {
    let n = 1;
    let res = true;
    assert_eq!(Solution::reordered_power_of2(n), res);
    let n = 10;
    let res = false;
    assert_eq!(Solution::reordered_power_of2(n), res);
    let n = 16;
    let res = true;
    assert_eq!(Solution::reordered_power_of2(n), res);
    let n = 24;
    let res = false;
    assert_eq!(Solution::reordered_power_of2(n), res);
    let n = 46;
    let res = true;
    assert_eq!(Solution::reordered_power_of2(n), res);
}
