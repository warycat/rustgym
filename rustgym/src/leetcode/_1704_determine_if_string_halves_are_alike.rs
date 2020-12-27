struct Solution;

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    fn halves_are_alike(s: String) -> bool {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let vowel: HashSet<char> =
            HashSet::from_iter(vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']);
        let left = s[..n / 2].iter().filter(|c| vowel.contains(c)).count();
        let right = s[n / 2..].iter().filter(|c| vowel.contains(c)).count();
        left == right
    }
}

#[test]
fn test() {
    let s = "book".to_string();
    let res = true;
    assert_eq!(Solution::halves_are_alike(s), res);
    let s = "textbook".to_string();
    let res = false;
    assert_eq!(Solution::halves_are_alike(s), res);
    let s = "MerryChristmas".to_string();
    let res = false;
    assert_eq!(Solution::halves_are_alike(s), res);
    let s = "AbCdEfGh".to_string();
    let res = true;
    assert_eq!(Solution::halves_are_alike(s), res);
}
