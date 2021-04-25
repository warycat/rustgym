struct Solution;

use std::collections::HashSet;

impl Solution {
    fn check_if_pangram(sentence: String) -> bool {
        let mut hs: HashSet<char> = HashSet::new();
        for c in sentence.chars() {
            hs.insert(c);
        }
        hs.len() == 26
    }
}

#[test]
fn test() {
    let sentence = "thequickbrownfoxjumpsoverthelazydog".to_string();
    let res = true;
    assert_eq!(Solution::check_if_pangram(sentence), res);
    let sentence = "leetcode".to_string();
    let res = false;
    assert_eq!(Solution::check_if_pangram(sentence), res);
}
