struct Solution;

use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::Hasher;

impl Solution {
    fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        let mut hs: HashSet<u64> = HashSet::new();
        for w in &words {
            let mut hasher = DefaultHasher::new();
            for b in w.bytes() {
                hasher.write_u8(b);
            }
            hs.insert(hasher.finish());
        }
        let mut res = vec![];
        for w in words {
            let s: Vec<u8> = w.bytes().collect();
            let n = s.len();
            if Self::dfs(0, 0, &hs, &s, n) {
                res.push(w);
            }
        }
        res
    }

    fn dfs(start: usize, k: usize, hs: &HashSet<u64>, s: &[u8], n: usize) -> bool {
        if start == n {
            k >= 2
        } else {
            let mut hasher = DefaultHasher::new();
            for i in start..n {
                hasher.write_u8(s[i]);
                if hs.contains(&hasher.finish()) && Self::dfs(i + 1, k + 1, hs, s, n) {
                    return true;
                }
            }
            false
        }
    }
}

#[test]
fn test() {
    let words = vec_string![
        "cat",
        "cats",
        "catsdogcats",
        "dog",
        "dogcatsdog",
        "hippopotamuses",
        "rat",
        "ratcatdogcat"
    ];
    let res = vec_string!["catsdogcats", "dogcatsdog", "ratcatdogcat"];
    assert_eq!(Solution::find_all_concatenated_words_in_a_dict(words), res);
}
