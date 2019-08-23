struct Solution;

impl Solution {
    fn bold_words(words: Vec<String>, s: String) -> String {
        let n = s.len();
        let mut bold: Vec<bool> = vec![false; n];
        let mut res: String = "".to_string();
        for word in words {
            let w = word.len();
            for i in 0..=(n - w) {
                let ss = &s[i..i + w];
                if ss == &word {
                    for j in 0..w {
                        bold[i + j] = true;
                    }
                }
            }
        }
        let s: Vec<char> = s.chars().collect();
        for i in 0..n {
            if bold[i] && (i == 0 || !bold[i - 1]) {
                res += "<b>";
            }
            res.push(s[i]);
            if bold[i] && (i == n - 1 || !bold[i + 1]) {
                res += "</b>";
            }
        }
        res
    }
}

#[test]
fn test() {
    let words: Vec<String> = ["ab", "bc"].iter().map(|s| s.to_string()).collect();
    let s = "aabcd".to_string();
    let res = "a<b>abc</b>d".to_string();
    assert_eq!(Solution::bold_words(words, s), res);
    let words: Vec<String> = ["e", "cab", "de", "cc", "db"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let s = "cbccceeead".to_string();
    let res = "cb<b>ccceee</b>ad".to_string();
    assert_eq!(Solution::bold_words(words, s), res);
    let words: Vec<String> = ["ccb", "b", "d", "cba", "dc"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let s = "eeaadadadc".to_string();
    let res = "eeaa<b>d</b>a<b>d</b>a<b>dc</b>".to_string();
    assert_eq!(Solution::bold_words(words, s), res);
}
