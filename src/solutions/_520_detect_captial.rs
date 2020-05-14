struct Solution;

impl Solution {
    fn detect_capital_use(word: String) -> bool {
        let n = word.len();
        if n <= 1 {
            return true;
        }
        let word: Vec<char> = word.chars().collect();
        let first_is_lowercase: bool = word[0].is_lowercase();
        if first_is_lowercase {
            for i in 1..n {
                if word[i].is_uppercase() {
                    return false;
                }
            }
        } else {
            let mut prev: Option<bool> = None;
            for i in 1..n {
                if let Some(prev_case) = prev {
                    if prev_case != word[i].is_uppercase() {
                        return false;
                    }
                } else {
                    prev = Some(word[i].is_uppercase());
                }
            }
        }
        true
    }
}

#[test]
fn test() {
    let word = "USA".to_string();
    assert_eq!(Solution::detect_capital_use(word), true);
    let word = "FlaG".to_string();
    assert_eq!(Solution::detect_capital_use(word), false);
}
