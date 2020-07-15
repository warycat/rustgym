struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut dict: HashSet<String> = HashSet::new();
        for word in word_list {
            dict.insert(word);
        }
        if !dict.contains(&end_word) {
            return vec![];
        }
        let set1: HashSet<String> = HashSet::from_iter(vec![begin_word.to_string()]);
        let set2: HashSet<String> = HashSet::from_iter(vec![end_word.to_string()]);
        let mut map: HashMap<String, HashSet<String>> = HashMap::new();
        Self::bfs(set1, set2, false, &mut map, &mut dict);
        let mut path = vec![begin_word.to_string()];
        let mut res = vec![];
        Self::dfs(&begin_word, &mut path, &mut res, &map, &end_word);
        res
    }

    fn bfs(
        set1: HashSet<String>,
        set2: HashSet<String>,
        flipped: bool,
        map: &mut HashMap<String, HashSet<String>>,
        dict: &mut HashSet<String>,
    ) {
        if set1.is_empty() {
            return;
        }
        if set1.len() > set2.len() {
            Self::bfs(set2, set1, !flipped, map, dict);
            return;
        }
        for s in set1.iter() {
            dict.remove(s);
        }
        for s in set2.iter() {
            dict.remove(s);
        }
        let mut next: HashSet<String> = HashSet::new();
        let mut done = false;
        for word in set1.iter() {
            for next_word in Self::connected_words(&word.to_string()) {
                let key = if flipped {
                    next_word.to_string()
                } else {
                    word.to_string()
                };
                let value = if flipped {
                    word.to_string()
                } else {
                    next_word.to_string()
                };
                if set2.contains(&next_word) {
                    done = true;
                    map.entry(key).or_default().insert(value);
                } else if dict.contains(&next_word) {
                    next.insert(next_word);
                    map.entry(key).or_default().insert(value);
                }
            }
        }
        if !done {
            Self::bfs(set2, next, !flipped, map, dict);
        }
    }

    fn dfs(
        start: &str,
        path: &mut Vec<String>,
        all: &mut Vec<Vec<String>>,
        map: &HashMap<String, HashSet<String>>,
        end: &str,
    ) {
        if start == end {
            all.push(path.to_vec());
        } else {
            if let Some(nei) = map.get(start) {
                for next in nei.iter() {
                    path.push(next.to_string());
                    Self::dfs(next, path, all, map, end);
                    path.pop();
                }
            }
        }
    }

    fn connected_words(word: &str) -> Vec<String> {
        let n = word.len();
        let mut res = vec![];
        for i in 0..n {
            let mut s: Vec<char> = word.chars().collect();
            for j in 0..26 {
                let c = (b'a' + j as u8) as char;
                s[i] = c;
                let new_word: String = s.iter().collect();
                res.push(new_word);
            }
        }
        res
    }
}

#[test]
fn test() {
    let begin_word = "hit".to_string();
    let end_word = "cog".to_string();
    let word_list = vec_string!["hot", "dot", "dog", "lot", "log", "cog"];
    let mut res = vec_vec_string![
        ["hit", "hot", "dot", "dog", "cog"],
        ["hit", "hot", "lot", "log", "cog"]
    ];
    let mut ans = Solution::find_ladders(begin_word, end_word, word_list);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);

    let begin_word = "hit".to_string();
    let end_word = "cog".to_string();
    let word_list = vec_string!["hot", "dot", "dog", "lot", "log"];
    let res = vec_vec_string![];
    assert_eq!(Solution::find_ladders(begin_word, end_word, word_list), res);

    let begin_word = "a".to_string();
    let end_word = "c".to_string();
    let word_list = vec_string!["a", "b", "c"];
    let mut res = vec_vec_string![["a", "c"]];
    let mut ans = Solution::find_ladders(begin_word, end_word, word_list);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}
