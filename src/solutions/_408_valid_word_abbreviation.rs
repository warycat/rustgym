struct Solution;

impl Solution {
    fn valid_word_abbreviation(word: String, abbr: String) -> bool {
        let mut i = 0;
        let mut j = 0;
        let n = word.len();
        let m = abbr.len();
        let word: Vec<char> = word.chars().collect();
        let abbr: Vec<char> = abbr.chars().collect();
        while i < n && j < m {
            if abbr[j].is_alphabetic() {
                if word[i] != abbr[j] {
                    return false;
                }
                i += 1;
                j += 1;
            } else {
                let mut v = 0;
                while j < m && abbr[j].is_numeric() {
                    if v == 0 && abbr[j] == '0' {
                        return false;
                    }
                    v *= 10;
                    v += (abbr[j] as u8 - b'0') as usize;
                    j += 1;
                }
                i += v;
            }
        }
        i == n && j == m
    }
}

#[test]
fn test() {
    let word = "internationalization".to_string();
    let abbr = "i12iz4n".to_string();
    assert_eq!(Solution::valid_word_abbreviation(word, abbr), true);
    let word = "apple".to_string();
    let abbr = "a2e".to_string();
    assert_eq!(Solution::valid_word_abbreviation(word, abbr), false);
}
