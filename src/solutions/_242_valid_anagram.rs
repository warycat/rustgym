struct Solution;

impl Solution {
    fn is_anagram(s: String, t: String) -> bool {
        let mut s: Vec<char> = s.chars().collect();
        let mut t: Vec<char> = t.chars().collect();
        s.sort_unstable();
        t.sort_unstable();
        s == t
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
        true
    );
    assert_eq!(
        Solution::is_anagram("rat".to_string(), "car".to_string()),
        false
    );
}
