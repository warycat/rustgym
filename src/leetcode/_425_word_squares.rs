struct Solution;

use std::collections::HashMap;

impl Solution {
    fn word_squares(words: Vec<String>) -> Vec<Vec<String>> {
        let n = words.len();
        let m = words[0].len();
        let words: Vec<Vec<char>> = words.into_iter().map(|s| s.chars().collect()).collect();
        let mut indexes: HashMap<Vec<char>, Vec<&[char]>> = HashMap::new();
        for word in words.iter() {
            for end in 0..=m {
                indexes.entry(word[..end].to_vec()).or_default().push(word);
            }
        }
        let mut cur = vec![];
        let mut res = vec![];
        Self::dfs(0, &mut cur, &mut res, &indexes, m, n);
        res
    }

    fn dfs<'a>(
        start: usize,
        cur: &mut Vec<&'a [char]>,
        res: &mut Vec<Vec<String>>,
        indexes: &HashMap<Vec<char>, Vec<&'a [char]>>,
        m: usize,
        n: usize,
    ) {
        if start == m {
            let square = (0..m).map(|i| cur[i].iter().collect()).collect();
            res.push(square);
        } else {
            let mut prefix = vec![];
            for i in 0..start {
                prefix.push(cur[i][start]);
            }
            if let Some(candidates) = indexes.get(&prefix) {
                for word in candidates {
                    cur.push(word);
                    Self::dfs(start + 1, cur, res, indexes, m, n);
                    cur.pop();
                }
            }
        }
    }
}

#[test]
fn test() {
    let words = vec_string!["area", "lead", "wall", "lady", "ball"];
    let res = vec_vec_string![
        ["wall", "area", "lead", "lady"],
        ["ball", "area", "lead", "lady"]
    ];
    assert_eq!(Solution::word_squares(words), res);
    let words = vec_string!["abat", "baba", "atan", "atal"];
    let res = vec_vec_string![
        ["baba", "abat", "baba", "atan"],
        ["baba", "abat", "baba", "atal"]
    ];
    assert_eq!(Solution::word_squares(words), res);
}
