struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut id: HashMap<String, usize> = HashMap::new();
        let n = words.len();
        let mut res = HashSet::new();
        for i in 0..n {
            id.insert(words[i].to_string(), i);
        }
        for i in 0..n {
            let k = words[i].len();
            for mid in 0..=k {
                let left: String = words[i][0..mid].to_string();
                let right: String = words[i][mid..].to_string();
                if Self::is_palindrome(&left) {
                    let right_r: String = right.chars().rev().collect();
                    if let Some(&j) = id.get(&right_r) {
                        if i != j {
                            res.insert(vec![j as i32, i as i32]);
                        }
                    }
                }
                if Self::is_palindrome(&right) {
                    let left_r: String = left.chars().rev().collect();
                    if let Some(&j) = id.get(&left_r) {
                        if i != j {
                            res.insert(vec![i as i32, j as i32]);
                        }
                    }
                }
            }
        }
        res.into_iter().collect()
    }

    fn is_palindrome(s: &str) -> bool {
        !s.chars().zip(s.chars().rev()).any(|(a, b)| a != b)
    }
}

#[test]
fn test() {
    let words = vec_string!["abcd", "dcba", "lls", "s", "sssll"];
    let mut res = vec![[0, 1], [1, 0], [3, 2], [2, 4]];
    let mut ans = Solution::palindrome_pairs(words);
    res.sort_unstable();
    ans.sort_unstable();
    assert_eq!(ans, res);
    let words = vec_string!["bat", "tab", "cat"];
    let mut res = vec![[0, 1], [1, 0]];
    let mut ans = Solution::palindrome_pairs(words);
    res.sort_unstable();
    ans.sort_unstable();
    assert_eq!(ans, res);
}
