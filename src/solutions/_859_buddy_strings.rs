struct Solution;

use std::collections::HashSet;

impl Solution {
    fn buddy_strings(a: String, b: String) -> bool {
        let a: Vec<char> = a.chars().collect();
        let b: Vec<char> = b.chars().collect();
        let n = a.len();
        let m = b.len();
        if n != m {
            return false;
        }
        if a == b {
            let mut hs: HashSet<char> = HashSet::new();
            let mut sum = 0;
            for &c in &a {
                if !hs.insert(c) {
                    sum += 1;
                }
            }
            sum != 0
        } else {
            let mut pair: Vec<usize> = vec![];
            for i in 0..n {
                if a[i] != b[i] {
                    pair.push(i);
                }
            }
            if pair.len() == 2 {
                let i = pair[0];
                let j = pair[1];
                a[i] == b[j] && a[j] == b[i]
            } else {
                false
            }
        }
    }
}

#[test]
fn test() {
    let a = "ab".to_string();
    let b = "ba".to_string();
    assert_eq!(Solution::buddy_strings(a, b), true);
    let a = "ab".to_string();
    let b = "ab".to_string();
    assert_eq!(Solution::buddy_strings(a, b), false);
    let a = "aa".to_string();
    let b = "aa".to_string();
    assert_eq!(Solution::buddy_strings(a, b), true);
    let a = "aaaaaaabc".to_string();
    let b = "aaaaaaacb".to_string();
    assert_eq!(Solution::buddy_strings(a, b), true);
    let a = "".to_string();
    let b = "aa".to_string();
    assert_eq!(Solution::buddy_strings(a, b), false);
}
