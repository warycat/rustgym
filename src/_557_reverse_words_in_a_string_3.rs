struct Solution;

impl Solution {
    fn reverse_words(s: String) -> String {
        let words: Vec<String> = s
            .split_whitespace()
            .map(|s| s.chars().rev().collect())
            .collect();
        let res: String = words.join(" ");
        res
    }
}

#[test]
fn test() {
    let s = "Let's take LeetCode contest".to_string();
    let r = "s'teL ekat edoCteeL tsetnoc".to_string();
    assert_eq!(Solution::reverse_words(s), r);
}
