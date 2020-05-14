struct Solution;

use std::collections::HashMap;

impl Solution {
    fn str_2_hs(s: &str) -> HashMap<char, i32> {
        let mut hs: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *hs.entry(c).or_default() += 1;
        }
        hs
    }

    fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let chars = Self::str_2_hs(&chars);
        let mut sum = 0;
        for w in words {
            let mut chars = chars.clone();
            let mut valid = true;
            for c in w.chars() {
                let count = chars.entry(c).or_default();
                *count -= 1;
                if *count < 0 {
                    valid = false;
                    break;
                }
            }
            if valid {
                sum += w.len();
            }
        }
        sum as i32
    }
}

#[test]
fn test() {
    let words = vec_string!["cat", "bt", "hat", "tree"];
    let chars = "atach".to_string();
    assert_eq!(Solution::count_characters(words, chars), 6);
    let words = vec_string!["hello", "world", "leetcode"];
    let chars = "welldonehoneyr".to_string();
    assert_eq!(Solution::count_characters(words, chars), 10);
}
