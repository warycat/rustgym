struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let p: String = paragraph
            .chars()
            .map(|c| {
                if c.is_alphabetic() {
                    c.to_ascii_lowercase()
                } else {
                    ' '
                }
            })
            .collect();
        let mut hm: HashMap<String, usize> = HashMap::new();
        for word in p.split_whitespace() {
            let e = hm.entry(word.to_string()).or_default();
            *e += 1;
        }
        let mut max = 0;
        let mut res = "".to_string();
        let banned: HashSet<String> = banned.into_iter().collect();
        for word in p.split_whitespace() {
            if !banned.contains(word) {
                if let Some(&count) = hm.get(word) {
                    if count > max {
                        max = count;
                        res = word.to_string();
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let paragraph: String = "Bob hit a ball, the hit BALL flew far after it was hit.".to_string();
    let banned: Vec<String> = vec_string!["hit"];
    assert_eq!(
        Solution::most_common_word(paragraph, banned),
        "ball".to_string()
    );
    let paragraph: String = "a".to_string();
    let banned: Vec<String> = vec![];
    assert_eq!(
        Solution::most_common_word(paragraph, banned),
        "a".to_string()
    );
}
