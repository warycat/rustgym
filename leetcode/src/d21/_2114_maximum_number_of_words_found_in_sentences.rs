struct Solution;

impl Solution {
    fn most_words_found(sentences: Vec<String>) -> i32 {
        sentences
            .iter()
            .map(|s| s.split_whitespace().map(|_s| 1).sum())
            .max()
            .unwrap()
    }
}

#[test]
fn test() {
    let sentences = vec_string![
        "alice and bob love leetcode",
        "i think so too",
        "this is great thanks very much"
    ];
    let res = 6;
    assert_eq!(Solution::most_words_found(sentences), res);
    let sentences = vec_string!["please wait", "continue to fight", "continue to win"];
    let res = 3;
    assert_eq!(Solution::most_words_found(sentences), res);
}
