struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    fn are_occurrences_equal(s: String) -> bool {
        let mut hm: HashMap<char, usize> = HashMap::new();
        for c in s.chars() {
            *hm.entry(c).or_default() += 1;
        }
        let mut hs: HashSet<usize> = HashSet::new();
        for v in hm.values() {
            hs.insert(*v);
        }
        hs.len() == 1
    }
}

#[test]
fn test() {
    let s = "abacbc".to_string();
    let res = true;
    assert_eq!(Solution::are_occurrences_equal(s), res);
    let s = "aaabb".to_string();
    let res = false;
    assert_eq!(Solution::are_occurrences_equal(s), res);
}
