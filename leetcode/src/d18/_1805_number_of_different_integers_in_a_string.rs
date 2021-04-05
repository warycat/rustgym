struct Solution;

use std::collections::HashSet;

impl Solution {
    fn num_different_integers(word: String) -> i32 {
        let word: String = word
            .chars()
            .map(|c| if c.is_numeric() { c } else { ' ' })
            .collect();
        let mut hs: HashSet<String> = HashSet::new();
        let words: Vec<Vec<char>> = word
            .split_whitespace()
            .map(|s| s.chars().rev().collect())
            .collect();
        for w in words {
            let mut wc = w.clone();
            while let Some(c) = wc.pop() {
                if c == '0' && w.last() == Some(&'0') {
                    continue;
                } else {
                    if c != '0' {
                        wc.push(c);
                    }
                    break;
                }
            }
            let s: String = wc.iter().collect();
            hs.insert(s);
        }
        hs.len() as i32
    }
}

#[test]
fn test() {
    let word = "a123bc34d8ef34".to_string();
    let res = 3;
    assert_eq!(Solution::num_different_integers(word), res);
    let word = "leet1234code234".to_string();
    let res = 2;
    assert_eq!(Solution::num_different_integers(word), res);
    let word = "a1b01c001".to_string();
    let res = 1;
    assert_eq!(Solution::num_different_integers(word), res);
}
