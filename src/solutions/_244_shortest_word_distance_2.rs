use std::collections::HashMap;
use std::usize;

struct WordDistance {
    indexes: HashMap<String, Vec<usize>>,
}

impl WordDistance {
    fn new(words: Vec<String>) -> Self {
        let mut indexes: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, word) in words.into_iter().enumerate() {
            indexes.entry(word).or_default().push(i);
        }
        WordDistance { indexes }
    }
    fn shortest(&self, word1: String, word2: String) -> i32 {
        let indexes1 = &self.indexes[&word1];
        let indexes2 = &self.indexes[&word2];
        let n1 = indexes1.len();
        let n2 = indexes2.len();
        let mut i1 = 0;
        let mut i2 = 0;
        let mut min = usize::MAX;
        while i1 < n1 && i2 < n2 {
            let v1 = indexes1[i1];
            let v2 = indexes2[i2];
            if v1 < v2 {
                min = usize::min(v2 - v1, min);
                i1 += 1;
            } else {
                min = usize::min(v1 - v2, min);
                i2 += 1;
            }
        }
        min as i32
    }
}

#[test]
fn test() {
    let words = vec_string!["practice", "makes", "perfect", "coding", "makes"];
    let obj = WordDistance::new(words);
    let word1 = "coding".to_string();
    let word2 = "practice".to_string();
    let res = 3;
    assert_eq!(obj.shortest(word1, word2), res);
    let word1 = "makes".to_string();
    let word2 = "coding".to_string();
    let res = 1;
    assert_eq!(obj.shortest(word1, word2), res);
}
