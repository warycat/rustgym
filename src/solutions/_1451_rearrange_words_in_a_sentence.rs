struct Solution;

impl Solution {
    fn arrange_words(text: String) -> String {
        let mut words: Vec<(usize, usize, String)> = text
            .split_whitespace()
            .enumerate()
            .map(|(i, s)| (s.len(), i, s.to_lowercase()))
            .collect();
        words.sort();
        let words: Vec<String> = words.into_iter().map(|(_, _, s)| s).collect();
        let mut res = words.join(" ");
        res[0..1].make_ascii_uppercase();
        res
    }
}

#[test]
fn test() {
    let text = "Leetcode is cool".to_string();
    let res = "Is cool leetcode".to_string();
    assert_eq!(Solution::arrange_words(text), res);
    let text = "Keep calm and code on".to_string();
    let res = "On and keep calm code".to_string();
    assert_eq!(Solution::arrange_words(text), res);
    let text = "To be or not to be".to_string();
    let res = "To be or to be not".to_string();
    assert_eq!(Solution::arrange_words(text), res);
}
