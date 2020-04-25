struct Solution;
use std::mem::swap;

impl Solution {
    fn reformat(s: String) -> String {
        let mut chars: Vec<char> = vec![];
        let mut digits: Vec<char> = vec![];
        let mut res: Vec<char> = vec![];
        for c in s.chars() {
            if c.is_digit(10) {
                digits.push(c);
            } else {
                chars.push(c);
            }
        }
        let mut iter;
        let mut next_iter;
        if digits.len() >= chars.len() {
            if digits.len() > chars.len() + 1 {
                return "".to_string();
            } else {
                iter = digits.iter();
                next_iter = chars.iter();
            }
        } else {
            if chars.len() > digits.len() + 1 {
                return "".to_string();
            } else {
                iter = chars.iter();
                next_iter = digits.iter();
            }
        }
        while let Some(c) = iter.next() {
            res.push(*c);
            swap(&mut iter, &mut next_iter);
        }
        res.into_iter().collect()
    }
}

#[test]
fn test() {
    let s = "a0b1c2".to_string();
    let res = "0a1b2c".to_string();
    assert_eq!(Solution::reformat(s), res);
    let s = "leetcode".to_string();
    let res = "".to_string();
    assert_eq!(Solution::reformat(s), res);
    let s = "1229857369".to_string();
    let res = "".to_string();
    assert_eq!(Solution::reformat(s), res);
    let s = "covid2019".to_string();
    let res = "c2o0v1i9d".to_string();
    assert_eq!(Solution::reformat(s), res);
    let s = "ab123".to_string();
    let res = "1a2b3".to_string();
    assert_eq!(Solution::reformat(s), res);
}
