struct Solution;

use std::collections::VecDeque;

impl Solution {
    fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let words1: Vec<String> = sentence1
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let words2: Vec<String> = sentence2
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let mut q1 = VecDeque::from(words1);
        let mut q2 = VecDeque::from(words2);
        while let (Some(w1), Some(w2)) = (q1.front(), q2.front()) {
            if w1 == w2 {
                q1.pop_front();
                q2.pop_front();
            } else {
                break;
            }
        }
        while let (Some(w1), Some(w2)) = (q1.back(), q2.back()) {
            if w1 == w2 {
                q1.pop_back();
                q2.pop_back();
            } else {
                break;
            }
        }
        q1.is_empty() || q2.is_empty()
    }
}

#[test]
fn test() {
    let sentence1 = "My name is Haley".to_string();
    let sentence2 = "My Haley".to_string();
    let res = true;
    assert_eq!(Solution::are_sentences_similar(sentence1, sentence2), res);
    let sentence1 = "of".to_string();
    let sentence2 = "A lot of words".to_string();
    let res = false;
    assert_eq!(Solution::are_sentences_similar(sentence1, sentence2), res);
    let sentence1 = "Eating right now".to_string();
    let sentence2 = "Eating".to_string();
    let res = true;
    assert_eq!(Solution::are_sentences_similar(sentence1, sentence2), res);
    let sentence1 = "Luky".to_string();
    let sentence2 = "Lucccky".to_string();
    let res = false;
    assert_eq!(Solution::are_sentences_similar(sentence1, sentence2), res);
}
