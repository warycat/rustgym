struct Solution;

impl Solution {
    fn sort_sentence(s: String) -> String {
        let mut pairs: Vec<(String, String)> = s
            .split_whitespace()
            .map(|s| {
                let n = s.len();
                let p = s[n - 1..].to_string();
                let word = s[0..n - 1].to_string();
                (p, word)
            })
            .collect();
        pairs.sort_unstable();
        let words: Vec<String> = pairs.into_iter().map(|(_, word)| word).collect();
        words.join(" ")
    }
}

#[test]
fn test() {
    let s = "is2 sentence4 This1 a3".to_string();
    let res = "This is a sentence".to_string();
    assert_eq!(Solution::sort_sentence(s), res);
    let s = "Myself2 Me1 I4 and3".to_string();
    let res = "Me Myself and I".to_string();
    assert_eq!(Solution::sort_sentence(s), res);
}
