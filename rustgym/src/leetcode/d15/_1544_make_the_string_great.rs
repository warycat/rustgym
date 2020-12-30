struct Solution;

impl Solution {
    fn make_good(s: String) -> String {
        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            if let Some(&last) = stack.last() {
                if c.is_uppercase() && last.is_lowercase() && c.to_ascii_lowercase() == last {
                    stack.pop();
                    continue;
                }
                if c.is_lowercase() && last.is_uppercase() && c.to_ascii_uppercase() == last {
                    stack.pop();
                    continue;
                }
            }
            stack.push(c);
        }
        stack.into_iter().collect()
    }
}

#[test]
fn test() {
    let s = "leEeetcode".to_string();
    let res = "leetcode".to_string();
    assert_eq!(Solution::make_good(s), res);
    let s = "abBAcC".to_string();
    let res = "".to_string();
    assert_eq!(Solution::make_good(s), res);
    let s = "s".to_string();
    let res = "s".to_string();
    assert_eq!(Solution::make_good(s), res);
}
