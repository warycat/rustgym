struct Solution;

impl Solution {
    fn remove_vowels(s: String) -> String {
        s.chars()
            .filter(|&c| !matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
            .collect()
    }
}

#[test]
fn test() {
    let s = "leetcodeisacommunityforcoders".to_string();
    let t = "ltcdscmmntyfrcdrs".to_string();
    assert_eq!(Solution::remove_vowels(s), t);
}
