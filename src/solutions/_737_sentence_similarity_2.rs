struct Solution;

use std::collections::HashMap;

struct UnionFind {
    parent: Vec<usize>,
    n: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        UnionFind { parent, n }
    }

    fn find(&mut self, i: usize) -> usize {
        let j = self.parent[i];
        if i == j {
            j
        } else {
            let k = self.find(j);
            self.parent[i] = k;
            k
        }
    }

    fn union(&mut self, i: usize, j: usize) {
        let i = self.find(i);
        let j = self.find(j);
        if i != j {
            self.parent[i] = j;
            self.n -= 1;
        }
    }
}

impl Solution {
    fn are_sentences_similar_two(
        words1: Vec<String>,
        words2: Vec<String>,
        pairs: Vec<Vec<String>>,
    ) -> bool {
        if words1.len() != words2.len() {
            return false;
        }
        let mut id: HashMap<String, usize> = HashMap::new();
        for w in &words1 {
            id.insert(w.to_string(), 0);
        }
        for w in &words2 {
            id.insert(w.to_string(), 0);
        }
        for pair in &pairs {
            id.insert(pair[0].to_string(), 0);
            id.insert(pair[1].to_string(), 0);
        }
        for (i, (_, v)) in id.iter_mut().enumerate() {
            *v = i;
        }
        let n = id.len();
        let mut uf = UnionFind::new(n);
        for pair in &pairs {
            uf.union(id[&pair[0]], id[&pair[1]]);
        }
        for (w1, w2) in words1.into_iter().zip(words2.into_iter()) {
            if uf.find(id[&w1]) != uf.find(id[&w2]) {
                return false;
            }
        }

        true
    }
}

#[test]
fn test() {
    let words1 = vec_string!["great", "acting", "skills"];
    let words2 = vec_string!["fine", "drama", "talent"];
    let pairs = vec_vec_string![
        ["great", "good"],
        ["fine", "good"],
        ["acting", "drama"],
        ["skills", "talent"]
    ];
    let res = true;
    assert_eq!(
        Solution::are_sentences_similar_two(words1, words2, pairs),
        res
    );
}
