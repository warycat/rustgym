struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    fn find_longest_word(s: String, d: Vec<String>) -> String {
        let s: Vec<char> = s.chars().collect();
        let mut queue: BinaryHeap<(usize, Reverse<String>)> =
            d.into_iter().map(|s| (s.len(), Reverse(s))).collect();
        while let Some((_, r)) = queue.pop() {
            let mut it = r.0.chars().peekable();
            for i in 0..s.len() {
                if let Some(&c) = it.peek() {
                    if c == s[i] {
                        it.next();
                    }
                } else {
                    break;
                }
            }
            if it.next().is_none() {
                return r.0;
            }
        }
        "".to_string()
    }
}

#[test]
fn test() {
    let s = "abpcplea".to_string();
    let d = vec_string!["ale", "apple", "monkey", "plea"];
    let res = "apple".to_string();
    assert_eq!(Solution::find_longest_word(s, d), res);
    let s = "abpcplea".to_string();
    let d = vec_string!["a", "b", "c"];
    let res = "a".to_string();
    assert_eq!(Solution::find_longest_word(s, d), res);
}
