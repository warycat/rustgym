struct Solution;

use std::collections::BTreeMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Count {
    even: BTreeMap<char, usize>,
    odd: BTreeMap<char, usize>,
}

impl Count {
    fn new(s: String) -> Self {
        let mut even: BTreeMap<char, usize> = BTreeMap::new();
        let mut odd: BTreeMap<char, usize> = BTreeMap::new();
        for (i, c) in s.chars().enumerate() {
            if i % 2 == 0 {
                *even.entry(c).or_default() += 1;
            } else {
                *odd.entry(c).or_default() += 1;
            }
        }
        Count { even, odd }
    }
}

impl Solution {
    fn num_special_equiv_groups(a: Vec<String>) -> i32 {
        let mut hs: HashSet<Count> = HashSet::new();
        for s in a {
            hs.insert(Count::new(s));
        }
        hs.len() as i32
    }
}

#[test]
fn test() {
    let a: Vec<String> = ["a", "b", "c", "a", "c", "c"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(Solution::num_special_equiv_groups(a), 3);
    let a: Vec<String> = ["aa", "bb", "ab", "ba"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(Solution::num_special_equiv_groups(a), 4);
    let a: Vec<String> = ["abc", "acb", "bac", "bca", "cab", "cba"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(Solution::num_special_equiv_groups(a), 3);
    let a: Vec<String> = ["abcd", "cdab", "adcb", "cbad"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(Solution::num_special_equiv_groups(a), 1);
}
