struct Solution;

use std::collections::HashMap;

impl Solution {
    fn encode(s: String) -> String {
        let mut memo: HashMap<String, String> = HashMap::new();
        Self::dp(s, &mut memo)
    }

    fn dp(s: String, memo: &mut HashMap<String, String>) -> String {
        if let Some(res) = memo.get(&s) {
            return res.to_string();
        }
        let n = s.len();
        let mut res = s.to_string();
        for i in 1..n {
            if n % i == 0 {
                let k = n / i;
                if (0..k).all(|j| s[0..i] == s[j * i..(j + 1) * i]) {
                    let t = format!("{}[{}]", k, Self::dp(s[0..i].to_string(), memo));
                    if t.len() < res.len() {
                        res = t;
                    }
                }
            }
        }
        for i in 1..n {
            let t = format!(
                "{}{}",
                Self::dp(s[0..i].to_string(), memo),
                Self::dp(s[i..].to_string(), memo)
            );
            if t.len() < res.len() {
                res = t;
            }
        }
        memo.insert(s, res.to_string());
        res
    }
}

#[test]
fn test() {
    let s = "aaa".to_string();
    let res = "aaa".to_string();
    assert_eq!(Solution::encode(s), res);
    let s = "aaaaa".to_string();
    let res = "5[a]".to_string();
    assert_eq!(Solution::encode(s), res);
    let s = "aabcaabcd".to_string();
    let res = "2[aabc]d".to_string();
    assert_eq!(Solution::encode(s), res);
    let s = "abbbabbbcabbbabbbc".to_string();
    let res = "2[2[abbb]c]".to_string();
    assert_eq!(Solution::encode(s), res);
}
