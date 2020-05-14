struct Solution;

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    fn before_and_after_puzzles(phrases: Vec<String>) -> Vec<String> {
        let mut res: BTreeSet<String> = BTreeSet::new();
        let mut first: HashMap<String, HashSet<String>> = HashMap::new();
        let mut last: HashMap<String, HashSet<String>> = HashMap::new();
        for p in phrases {
            let n = p.len();
            let fp = if let Some(fp) = p.find(' ') { fp } else { n };
            let lp = if let Some(lp) = p.rfind(' ') {
                lp + 1
            } else {
                0
            };
            let fw = &p[0..fp];
            let lw = &p[lp..n];
            for pp in &*first.entry(lw.to_string()).or_default() {
                res.insert(format!("{}{}", p, pp));
            }
            for pp in &*last.entry(fw.to_string()).or_default() {
                res.insert(format!("{}{}", pp, p));
            }
            first
                .entry(fw.to_string())
                .or_default()
                .insert(p[fp..n].to_string());
            last.entry(lw.to_string())
                .or_default()
                .insert(p[0..lp].to_string());
        }
        res.into_iter().collect()
    }
}

#[test]
fn test() {
    let phrases: Vec<String> = vec_string!["writing code", "code rocks"];
    let res: Vec<String> = vec_string!["writing code rocks"];
    assert_eq!(Solution::before_and_after_puzzles(phrases), res);
    let phrases: Vec<String> = vec_string![
        "mission statement",
        "a quick bite to eat",
        "a chip off the old block",
        "chocolate bar",
        "mission impossible",
        "a man on a mission",
        "block party",
        "eat my words",
        "bar of soap"
    ];
    let res: Vec<String> = vec_string![
        "a chip off the old block party",
        "a man on a mission impossible",
        "a man on a mission statement",
        "a quick bite to eat my words",
        "chocolate bar of soap"
    ];
    assert_eq!(Solution::before_and_after_puzzles(phrases), res);
}
