struct Solution;
use std::collections::HashSet;

impl Solution {
    fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let broken_keys: HashSet<char> = broken_letters.chars().collect();
        let mut res = 0;
        'outer: for word in text.split_whitespace() {
            for c in word.chars() {
                if broken_keys.contains(&c) {
                    continue 'outer;
                }
            }
            res += 1;
        }
        res
    }
}

#[test]
fn test() {
    let text = "hello world".to_string();
    let broken_letters = "ad".to_string();
    let res = 1;
    assert_eq!(Solution::can_be_typed_words(text, broken_letters), res);
    let text = "leet code".to_string();
    let broken_letters = "lt".to_string();
    let res = 1;
    assert_eq!(Solution::can_be_typed_words(text, broken_letters), res);
    let text = "leet code".to_string();
    let broken_letters = "e".to_string();
    let res = 0;
    assert_eq!(Solution::can_be_typed_words(text, broken_letters), res);
}
