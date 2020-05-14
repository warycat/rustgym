struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    fn are_sentences_similar(
        words1: Vec<String>,
        words2: Vec<String>,
        pairs: Vec<Vec<String>>,
    ) -> bool {
        let mut hm: HashMap<String, HashSet<String>> = HashMap::new();
        for word in &words1 {
            let e = hm.entry(word.to_string()).or_default();
            e.insert(word.to_string());
        }
        for word in &words2 {
            let e = hm.entry(word.to_string()).or_default();
            e.insert(word.to_string());
        }
        for pair in &pairs {
            let a = pair[0].to_string();
            let b = pair[1].to_string();
            let e = hm.entry(a.to_string()).or_default();
            e.insert(b.to_string());
            let e = hm.entry(b.to_string()).or_default();
            e.insert(a.to_string());
        }
        let n = words1.len();
        let m = words2.len();
        if n != m {
            return false;
        }
        for i in 0..n {
            let set = &hm[&words1[i]];
            if !set.contains(&words2[i]) {
                return false;
            }
        }

        true
    }
}

#[test]
fn test() {
    let words1: Vec<String> = vec_string!["an", "extraordinary", "meal"];
    let words2: Vec<String> = vec_string!["one", "good", "dinner"];
    let pairs: Vec<Vec<String>> = vec_vec_string![
        ["great", "good"],
        ["extraordinary", "good"],
        ["well", "good"],
        ["wonderful", "good"],
        ["excellent", "good"],
        ["fine", "good"],
        ["nice", "good"],
        ["any", "one"],
        ["some", "one"],
        ["unique", "one"],
        ["the", "one"],
        ["an", "one"],
        ["single", "one"],
        ["a", "one"],
        ["truck", "car"],
        ["wagon", "car"],
        ["automobile", "car"],
        ["auto", "car"],
        ["vehicle", "car"],
        ["entertain", "have"],
        ["drink", "have"],
        ["eat", "have"],
        ["take", "have"],
        ["fruits", "meal"],
        ["brunch", "meal"],
        ["breakfast", "meal"],
        ["food", "meal"],
        ["dinner", "meal"],
        ["super", "meal"],
        ["lunch", "meal"],
        ["possess", "own"],
        ["keep", "own"],
        ["have", "own"],
        ["extremely", "very"],
        ["actually", "very"],
        ["really", "very"],
        ["super", "very"]
    ];
    assert_eq!(Solution::are_sentences_similar(words1, words2, pairs), true);
}
