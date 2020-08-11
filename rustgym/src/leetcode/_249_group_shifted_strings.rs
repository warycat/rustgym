struct Solution;
use std::collections::HashMap;

impl Solution {
    fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
        let mut hm: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
        for s in strings {
            let mut v: Vec<u8> = s.bytes().collect();
            v = v.iter().map(|b| (b + 26 - v[0]) % 26).collect();
            hm.entry(v).or_default().push(s);
        }
        hm.into_iter().map(|(_, v)| v).collect()
    }
}

#[test]
fn test() {
    let strings = vec_string!["abc", "bcd", "acef", "xyz", "az", "ba", "a", "z"];
    let mut res = vec_vec_string![["abc", "bcd", "xyz"], ["az", "ba"], ["acef"], ["a", "z"]];
    let mut ans = Solution::group_strings(strings);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}
