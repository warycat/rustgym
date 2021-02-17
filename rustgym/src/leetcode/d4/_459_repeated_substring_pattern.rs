struct Solution;

impl Solution {
    fn repeated_substring_pattern(s: String) -> bool {
        let mut t: String = "".to_string();
        let n = s.len();
        if n < 2 {
            return false;
        }
        t += &s;
        t += &s;
        t[1..2 * n - 1].contains(&s)
    }
}

#[test]
fn test() {
    let s = "abab".to_string();
    assert_eq!(Solution::repeated_substring_pattern(s), true);
    let s = "aba".to_string();
    assert_eq!(Solution::repeated_substring_pattern(s), false);
    let s = "abcabcabcabc".to_string();
    assert_eq!(Solution::repeated_substring_pattern(s), true);
}
