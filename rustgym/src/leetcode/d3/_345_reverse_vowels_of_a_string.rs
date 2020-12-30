struct Solution;

impl Solution {
    fn is_vowel(c: char) -> bool {
        matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
    }

    fn reverse_vowels(s: String) -> String {
        let mut a: Vec<char> = s.chars().collect();
        let n = a.len();
        if n == 0 {
            return "".to_string();
        }
        let mut l = 0;
        let mut r = n - 1;
        while l < r {
            if Self::is_vowel(a[l]) && Self::is_vowel(a[r]) {
                a.swap(l, r);
                l += 1;
                r -= 1;
            } else {
                if !Self::is_vowel(a[l]) {
                    l += 1;
                }
                if !Self::is_vowel(a[r]) {
                    r -= 1;
                }
            }
        }
        a.iter().collect()
    }
}

#[test]
fn test() {
    let s = "hello".to_string();
    let t = "holle".to_string();
    assert_eq!(Solution::reverse_vowels(s), t);
    let s = "leetcode".to_string();
    let t = "leotcede".to_string();
    assert_eq!(Solution::reverse_vowels(s), t);
    let s = "a.".to_string();
    let t = "a.".to_string();
    assert_eq!(Solution::reverse_vowels(s), t);
}
