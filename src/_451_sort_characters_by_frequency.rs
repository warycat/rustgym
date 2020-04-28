struct Solution;
use std::cmp::Reverse;
use std::collections::HashMap;

impl Solution {
    fn frequency_sort(s: String) -> String {
        let mut s: Vec<char> = s.chars().collect();
        let mut hm: HashMap<char, usize> = HashMap::new();
        for &c in &s {
            *hm.entry(c).or_default() += 1;
        }
        s.sort_unstable_by_key(|&c| (Reverse(hm[&c]), Reverse(c)));
        s.into_iter().collect()
    }
}

#[test]
fn test() {
    let s = "tree".to_string();
    let res = "eetr".to_string();
    assert_eq!(Solution::frequency_sort(s), res);
    let s = "cccaaa".to_string();
    let res = "cccaaa".to_string();
    assert_eq!(Solution::frequency_sort(s), res);
    let s = "Aabb".to_string();
    let res = "bbaA".to_string();
    assert_eq!(Solution::frequency_sort(s), res);
}
