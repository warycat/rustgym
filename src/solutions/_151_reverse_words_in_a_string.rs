struct Solution;

impl Solution {
    fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
}

#[test]
fn test() {
    let s: String = "the sky is blue".to_string();
    let res: String = "blue is sky the".to_string();
    assert_eq!(Solution::reverse_words(s), res);
}
