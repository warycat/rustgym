pub struct Solution;

impl Solution {
    pub fn remove_vowels(s: String) -> String {
        s.chars()
            .filter(|&c| match c {
                'a' | 'e' | 'i' | 'o' | 'u' => false,
                _ => true,
            })
            .collect()
    }
}

#[test]
fn test() {
    let s = "leetcodeisacommunityforcoders".to_string();
    let t = "ltcdscmmntyfrcdrs".to_string();
    assert_eq!(Solution::remove_vowels(s), t);
}
