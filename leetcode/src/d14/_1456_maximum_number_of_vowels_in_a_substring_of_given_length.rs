struct Solution;

impl Solution {
    fn max_vowels(s: String, k: i32) -> i32 {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let k = k as usize;
        let mut cur = 0;
        let mut res = 0;
        for i in 0..n {
            if Self::is_vowel(s[i]) {
                cur += 1;
            }
            if i >= k {
                if Self::is_vowel(s[i - k]) {
                    cur -= 1;
                }
            }
            res = res.max(cur);
        }
        res
    }

    fn is_vowel(c: char) -> bool {
        matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
    }
}

#[test]
fn test() {
    let s = "abciiidef".to_string();
    let k = 3;
    let res = 3;
    assert_eq!(Solution::max_vowels(s, k), res);
    let s = "aeiou".to_string();
    let k = 2;
    let res = 2;
    assert_eq!(Solution::max_vowels(s, k), res);
    let s = "leetcode".to_string();
    let k = 3;
    let res = 2;
    assert_eq!(Solution::max_vowels(s, k), res);
    let s = "rhythms".to_string();
    let k = 4;
    let res = 0;
    assert_eq!(Solution::max_vowels(s, k), res);
    let s = "tryhard".to_string();
    let k = 4;
    let res = 1;
    assert_eq!(Solution::max_vowels(s, k), res);
}
