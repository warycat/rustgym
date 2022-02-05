struct Solution;

impl Solution {
    fn count_vowel_substrings(word: String) -> i32 {
        let s: Vec<char> = word.chars().collect();
        let n = s.len();
        let mut res = 0;
        for i in 0..n {
            for j in i..n {
                if Solution::is_vowel(&s, i, j) {
                    res += 1;
                }
            }
        }
        res
    }

    fn is_vowel(s: &[char], start: usize, end: usize) -> bool {
        let mut ca = 0;
        let mut ce = 0;
        let mut ci = 0;
        let mut co = 0;
        let mut cu = 0;

        for i in start..=end {
            match s[i] {
                'a' => {
                    ca += 1;
                }
                'e' => {
                    ce += 1;
                }
                'i' => {
                    ci += 1;
                }
                'o' => {
                    co += 1;
                }
                'u' => {
                    cu += 1;
                }
                _ => {
                    return false;
                }
            }
        }
        ca >= 1 && ce >= 1 && ci >= 1 && co >= 1 && cu >= 1
    }
}

#[test]
fn test() {
    let word = "aeiouu".to_string();
    let res = 2;
    assert_eq!(Solution::count_vowel_substrings(word), res);
    let word = "unicornarihan".to_string();
    let res = 0;
    assert_eq!(Solution::count_vowel_substrings(word), res);
    let word = "cuaieuouac".to_string();
    let res = 7;
    assert_eq!(Solution::count_vowel_substrings(word), res);
}
