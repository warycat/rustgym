struct Solution;

use std::collections::HashMap;

impl Solution {
    fn word_pattern(pattern: String, string: String) -> bool {
        let chars: Vec<char> = pattern.chars().collect();
        let strings: Vec<String> = string.split_whitespace().map(|s| s.to_string()).collect();
        if chars.len() != strings.len() {
            return false;
        }
        let mut hm1: HashMap<char, String> = HashMap::new();
        let mut hm2: HashMap<String, char> = HashMap::new();
        for i in 0..chars.len() {
            let ch = chars[i];
            let string = strings[i].clone();
            if let Some(s) = hm1.get(&ch) {
                if *s != string {
                    return false;
                }
            } else {
                hm1.insert(ch, string.clone());
            }
            if let Some(c) = hm2.get(&string) {
                if *c != ch {
                    return false;
                }
            } else {
                hm2.insert(string.clone(), ch);
            }
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::word_pattern("abba".to_string(), "dog cat cat dog".to_string()),
        true
    );
    assert_eq!(
        Solution::word_pattern("abba".to_string(), "dog cat cat fish".to_string()),
        false
    );
    assert_eq!(
        Solution::word_pattern("aaaa".to_string(), "dog cat cat dog".to_string()),
        false
    );
    assert_eq!(
        Solution::word_pattern("abba".to_string(), "dog dog dog dog".to_string()),
        false
    );
}
