struct Solution;

use std::collections::BTreeMap;

impl Solution {
    fn uncommon_from_sentences(a: String, b: String) -> Vec<String> {
        let mut hs: BTreeMap<&str, i32> = BTreeMap::new();
        a.split_whitespace().for_each(|s| {
            *hs.entry(s).or_default() += 1;
        });
        b.split_whitespace().for_each(|s| {
            *hs.entry(s).or_default() += 1;
        });
        let mut res: Vec<String> = vec![];
        for (s, v) in hs {
            if v == 1 {
                res.push(s.to_string());
            }
        }
        res
    }
}

#[test]
fn test() {
    let a = "this apple is sweet".to_string();
    let b = "this apple is sour".to_string();
    let res: Vec<String> = vec_string!["sour", "sweet"];
    assert_eq!(Solution::uncommon_from_sentences(a, b), res);
    let a = "apple apple".to_string();
    let b = "banana".to_string();
    let res: Vec<String> = vec_string!["banana"];
    assert_eq!(Solution::uncommon_from_sentences(a, b), res);
}
