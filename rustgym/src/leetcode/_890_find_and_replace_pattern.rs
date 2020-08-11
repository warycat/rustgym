struct Solution;
use std::collections::HashMap;

trait Matches {
    fn matches(&self, pattern: &[char], n: usize) -> bool;
}

impl Matches for Vec<char> {
    fn matches(&self, pattern: &[char], n: usize) -> bool {
        if self.len() != n {
            return false;
        }
        let mut hm1: HashMap<char, char> = HashMap::new();
        let mut hm2: HashMap<char, char> = HashMap::new();
        for i in 0..n {
            let c1 = self[i];
            let c2 = pattern[i];
            if let Some(old) = hm1.insert(c1, c2) {
                if old != c2 {
                    return false;
                }
            }
            if let Some(old) = hm2.insert(c2, c1) {
                if old != c1 {
                    return false;
                }
            }
        }
        true
    }
}

impl Solution {
    fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let n = pattern.len();
        let pattern: Vec<char> = pattern.chars().collect();
        let mut res = vec![];
        for word in words {
            let word: Vec<char> = word.chars().collect();
            if word.matches(&pattern, n) {
                res.push(word.iter().collect());
            }
        }
        res
    }
}

#[test]
fn test() {
    let words = vec_string!["abc", "deq", "mee", "aqq", "dkd", "ccc"];
    let pattern = "abb".to_string();
    let res = vec_string!["mee", "aqq"];
    assert_eq!(Solution::find_and_replace_pattern(words, pattern), res);
}
