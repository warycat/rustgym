struct Solution;

use std::collections::HashMap;

impl Solution {
    fn find_words(words: Vec<String>) -> Vec<String> {
        let rows: Vec<String> = [
            "qwertyuiopQWERTYUIOP",
            "asdfghjklASDFGHJKL",
            "zxcvbnmZXCVBNM",
        ]
        .iter()
        .map(|v| (*v).to_string())
        .collect();
        let mut hm: HashMap<char, usize> = HashMap::new();
        for i in 0..3 {
            let row = &rows[i];
            for c in row.chars() {
                hm.insert(c, i);
            }
        }
        let mut res: Vec<String> = vec![];
        for word in words {
            let rows: Vec<usize> = word.chars().map(|c| *hm.get(&c).unwrap()).collect();
            if rows.windows(2).all(|w| w[0] == w[1]) {
                res.push(word);
            }
        }
        res
    }
}

#[test]
fn test() {
    let words: Vec<String> = vec_string!["Hello", "Alaska", "Dad", "Peace"];
    let res: Vec<String> = vec_string!["Alaska", "Dad"];
    assert_eq!(Solution::find_words(words), res);
}
