struct Solution;
use std::collections::HashSet;

impl Solution {
    fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut hs: HashSet<String> = HashSet::new();
        for s in folder {
            hs.insert(s);
        }
        let mut res = vec![];
        for s in &hs {
            if !Self::is_subfolder(s, &hs) {
                res.push(s.clone());
            }
        }
        res
    }

    fn is_subfolder(s: &str, hs: &HashSet<String>) -> bool {
        let n = s.len();
        for i in 0..n {
            if &s[i..=i] == "/" {
                if hs.contains(&s[0..i]) {
                    return true;
                }
            }
        }
        false
    }
}

#[test]
fn test() {
    let folder = vec_string!["/a", "/a/b", "/c/d", "/c/d/e", "/c/f"];
    let mut res = vec_string!["/a", "/c/d", "/c/f"];
    let mut ans = Solution::remove_subfolders(folder);
    ans.sort();
    res.sort();
    assert_eq!(ans, res);
    let folder = vec_string!["/a", "/a/b/c", "/a/b/d"];
    let mut res = vec_string!["/a"];
    let mut ans = Solution::remove_subfolders(folder);
    ans.sort();
    res.sort();
    assert_eq!(ans, res);
    let folder = vec_string!["/a/b/c", "/a/b/ca", "/a/b/d"];
    let mut res = vec_string!["/a/b/c", "/a/b/ca", "/a/b/d"];
    let mut ans = Solution::remove_subfolders(folder);
    ans.sort();
    res.sort();
    assert_eq!(ans, res);
}
