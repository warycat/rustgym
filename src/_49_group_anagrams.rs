struct Solution;

use std::collections::HashMap;

impl Solution {
    fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut group_hm: HashMap<Vec<char>, Vec<String>> = HashMap::new();
        for s in strs {
            let mut v: Vec<char> = s.chars().collect();
            v.sort_unstable();
            let group = group_hm.entry(v).or_default();
            group.push(s);
        }
        let mut res: Vec<Vec<String>> = vec![];
        for (_, mut v) in group_hm {
            v.sort();
            res.push(v.to_vec());
        }
        res.sort_by(|a, b| a.len().cmp(&b.len()));
        res
    }
}

#[test]
fn test() {
    let strs: Vec<String> = ["eat", "tea", "tan", "ate", "nat", "bat"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let res: Vec<Vec<String>> = [vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]
        .iter()
        .map(|v| v.iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(Solution::group_anagrams(strs), res);
}
