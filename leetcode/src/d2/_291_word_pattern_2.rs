struct Solution;
use std::collections::HashSet;

impl Solution {
    fn word_pattern_match(pattern: String, s: String) -> bool {
        let mut map: Vec<&str> = vec![""; 26];
        let mut set: HashSet<&str> = HashSet::new();
        let n = pattern.len();
        let pattern: Vec<char> = pattern.chars().collect();
        Self::dfs(0, &mut map, &mut set, &pattern, &s, n)
    }

    fn dfs<'a>(
        start: usize,
        map: &mut Vec<&'a str>,
        set: &mut HashSet<&'a str>,
        pattern: &[char],
        s: &'a str,
        n: usize,
    ) -> bool {
        if start == n {
            s.is_empty()
        } else {
            let i = (pattern[start] as u8 - b'a') as usize;
            if !map[i].is_empty() {
                let size = map[i].len();
                if s.starts_with(&map[i]) {
                    Self::dfs(start + 1, map, set, pattern, &s[size..], n)
                } else {
                    false
                }
            } else {
                let k = s.len();
                for size in 1..=k {
                    let t = &s[..size];
                    if !set.contains(t) {
                        map[i] = t;
                        set.insert(t);
                        if Self::dfs(start + 1, map, set, pattern, &s[size..], n) {
                            return true;
                        }
                        map[i] = "";
                        set.remove(t);
                    }
                }
                false
            }
        }
    }
}

#[test]
fn test() {
    let pattern = "abab".to_string();
    let s = "redblueredblue".to_string();
    let res = true;
    assert_eq!(Solution::word_pattern_match(pattern, s), res);
    let pattern = "aaaa".to_string();
    let s = "asdasdasdasd".to_string();
    let res = true;
    assert_eq!(Solution::word_pattern_match(pattern, s), res);
    let pattern = "aabb".to_string();
    let s = "xyzabcxzyabc".to_string();
    let res = false;
    assert_eq!(Solution::word_pattern_match(pattern, s), res);
}
