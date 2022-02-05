struct Solution;

impl Solution {
    fn capitalize_title(title: String) -> String {
        let words: Vec<String> = title
            .split_whitespace()
            .map(Solution::capitalize_word)
            .collect();
        words.join(" ")
    }

    fn capitalize_word(s: &str) -> String {
        let n = s.len();
        let mut res = "".to_string();
        for (i, c) in s.char_indices() {
            if i == 0 {
                if n <= 2 {
                    res.push(c.to_ascii_lowercase());
                } else {
                    res.push(c.to_ascii_uppercase());
                }
            } else {
                res.push(c.to_ascii_lowercase());
            }
        }
        res
    }
}

#[test]
fn test() {
    let title = "capiTalIze tHe titLe".to_string();
    let res = "Capitalize The Title".to_string();
    assert_eq!(Solution::capitalize_title(title), res);
    let title = "First leTTeR of EACH Word".to_string();
    let res = "First Letter of Each Word".to_string();
    assert_eq!(Solution::capitalize_title(title), res);
    let title = "i lOve leetcode".to_string();
    let res = "i Love Leetcode".to_string();
    assert_eq!(Solution::capitalize_title(title), res);
}
