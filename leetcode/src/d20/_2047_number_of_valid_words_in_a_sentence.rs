struct Solution;

impl Solution {
    fn count_valid_words(sentence: String) -> i32 {
        let mut res = 0;
        for word in sentence.split_whitespace() {
            if Solution::is_valid(word) {
                res += 1;
            }
        }
        res
    }

    fn is_valid(s: &str) -> bool {
        let n = s.len();
        let mut h_count = 0;
        let s: Vec<char> = s.chars().collect();
        for i in 0..n {
            let c = s[i];
            match c {
                'a'..='z' => {}
                ',' | '!' | '.' => {
                    if i != n - 1 {
                        return false;
                    }
                }
                '-' => {
                    h_count += 1;
                    if i == 0 || i == n - 1 {
                        return false;
                    }
                    if !(('a'..='z').contains(&s[i - 1]) && ('a'..='z').contains(&s[i + 1])) {
                        return false;
                    }
                }
                _ => {
                    return false;
                }
            };
        }
        if h_count > 1 {
            return false;
        }
        true
    }
}

#[test]
fn test() {
    let sentence = "cat and  dog".to_string();
    let res = 3;
    assert_eq!(Solution::count_valid_words(sentence), res);
    let sentence = "!this  1-s b8d!".to_string();
    let res = 0;
    assert_eq!(Solution::count_valid_words(sentence), res);
    let sentence = "alice and  bob are playing stone-game10".to_string();
    let res = 5;
    assert_eq!(Solution::count_valid_words(sentence), res);
}
