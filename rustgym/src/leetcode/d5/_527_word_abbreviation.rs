struct Solution;

use std::collections::HashMap;

impl Solution {
    fn words_abbreviation(dict: Vec<String>) -> Vec<String> {
        let mut count: HashMap<(&str, &str, usize), usize> = HashMap::new();
        for s in &dict {
            let n = s.len();
            let suffix = &s[n - 1..];
            for i in 1..n {
                let prefix = &s[..i];
                let k = n - i - 1;
                *count.entry((prefix, suffix, k)).or_default() += 1;
            }
        }
        let mut res = vec![];
        for s in &dict {
            let n = s.len();
            let suffix = &s[n - 1..];
            for i in 1..n {
                let prefix = &s[..i];
                let k = n - i - 1;
                if let Some(1) = count.get(&(prefix, suffix, k)) {
                    if k <= 1 {
                        res.push(s.to_string());
                    } else {
                        res.push(format!("{}{}{}", prefix, k, suffix));
                    }
                    break;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let dict = vec_string![
        "like",
        "god",
        "internal",
        "me",
        "internet",
        "interval",
        "intension",
        "face",
        "intrusion"
    ];
    let res =
        vec_string!["l2e", "god", "internal", "me", "i6t", "interval", "inte4n", "f2e", "intr4n"];
    assert_eq!(Solution::words_abbreviation(dict), res);
}
