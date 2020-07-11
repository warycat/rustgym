struct Solution;

use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::Hasher;

impl Solution {
    fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let n = s.len();
        let mut dict: HashSet<u64> = HashSet::new();
        let mut alphabet: Vec<bool> = vec![false; 256];
        for word in word_dict {
            let mut hasher = DefaultHasher::new();
            for b in word.bytes() {
                alphabet[b as usize] = true;
                hasher.write_u8(b);
            }
            dict.insert(hasher.finish());
        }
        let s: Vec<u8> = s.bytes().collect();
        for i in 0..n {
            if !alphabet[s[i] as usize] {
                return vec![];
            }
        }
        let mut cur = vec![];
        let mut res = vec![];
        Self::dfs(0, &mut cur, &mut res, &dict, &s, n);
        res
    }

    fn dfs(
        start: usize,
        cur: &mut Vec<(usize, usize)>,
        all: &mut Vec<String>,
        dict: &HashSet<u64>,
        s: &[u8],
        n: usize,
    ) {
        if start == n {
            let mut words = vec![];
            for &(l, r) in cur.iter() {
                let mut word = "".to_string();
                for i in l..=r {
                    word.push(s[i] as char);
                }
                words.push(word);
            }
            all.push(words.join(" "));
        }
        let mut hasher = DefaultHasher::new();
        for i in start..n {
            hasher.write_u8(s[i]);
            if dict.contains(&hasher.finish()) {
                cur.push((start, i));
                Self::dfs(i + 1, cur, all, dict, s, n);
                cur.pop();
            }
        }
    }
}

#[test]
fn test() {
    let s = "catsanddog".to_string();
    let word_dict = vec_string!["cat", "cats", "and", "sand", "dog"];
    let mut res = vec_string!["cats and dog", "cat sand dog"];
    let mut ans = Solution::word_break(s, word_dict);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let s = "pineapplepenapple".to_string();
    let word_dict = vec_string!["apple", "pen", "applepen", "pine", "pineapple"];
    let mut res = vec_string![
        "pine apple pen apple",
        "pineapple pen apple",
        "pine applepen apple"
    ];
    let mut ans = Solution::word_break(s, word_dict);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let s = "catsandog".to_string();
    let word_dict = vec_string!["cats", "dog", "sand", "and", "cat"];
    let mut res = vec_string![];
    let mut ans = Solution::word_break(s, word_dict);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}
