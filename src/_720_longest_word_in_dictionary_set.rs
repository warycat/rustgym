struct Solution;

use std::collections::HashSet;

impl Solution {
    fn longest_word(mut words: Vec<String>) -> String {
        words.sort_unstable();
        let mut hs: HashSet<&str> = HashSet::new();
        let mut max = "";
        for word in words.iter() {
            let n = word.len();
            if n == 1 || hs.contains(&word[0..n - 1]) {
                hs.insert(word);
                if word.len() > max.len() {
                    max = word;
                }
            }
        }
        max.to_string()
    }
}

#[test]
fn test() {
    let words: Vec<String> = ["a", "banana", "app", "appl", "ap", "apply", "apple"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let res = "apple".to_string();
    assert_eq!(Solution::longest_word(words), res);
}
