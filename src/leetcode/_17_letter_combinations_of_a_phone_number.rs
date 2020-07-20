struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dfs(
        hm: &HashMap<char, Vec<char>>,
        digits: &[char],
        s: &mut Vec<char>,
        res: &mut Vec<String>,
        index: usize,
    ) {
        if index == digits.len() {
            res.push(s.iter().collect());
        } else {
            let d = digits[index];
            for &c in hm[&d].iter() {
                s.push(c);
                Self::dfs(hm, digits, s, res, index + 1);
                s.pop();
            }
        }
    }
    fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let digits: Vec<char> = digits.chars().collect();
        let hm: HashMap<char, Vec<char>> = [
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ]
        .iter()
        .map(|(d, v)| (*d, v.chars().collect()))
        .collect();
        let mut s: Vec<char> = vec![];
        let mut res: Vec<String> = vec![];
        Self::dfs(&hm, &digits, &mut s, &mut res, 0);
        res
    }
}

#[test]
fn test() {
    let digits = "23".to_string();
    let res: Vec<String> = vec_string!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];
    assert_eq!(Solution::letter_combinations(digits), res);
}
