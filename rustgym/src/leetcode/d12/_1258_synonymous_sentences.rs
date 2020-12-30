struct Solution;
use std::collections::BTreeSet;
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

    fn union(&mut self, mut i: usize, mut j: usize) {
        i = self.find(i);
        j = self.find(j);
        if i != j {
            self.parent[i] = j;
        }
    }

    fn groups(&mut self) -> HashMap<usize, Vec<usize>> {
        let mut hm: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..self.n {
            let j = self.find(i);
            hm.entry(j).or_default().push(i);
        }
        hm
    }
}

impl Solution {
    fn generate_sentences(synonyms: Vec<Vec<String>>, text: String) -> Vec<String> {
        let mut bts: BTreeSet<String> = BTreeSet::new();
        for ss in &synonyms {
            bts.insert(ss[0].to_string());
            bts.insert(ss[1].to_string());
        }
        for s in text.split_whitespace() {
            bts.insert(s.to_string());
        }
        let n = bts.len();
        let mut uf = UnionFind::new(n);
        let mut words = vec![];
        let mut ids: HashMap<String, usize> = HashMap::new();
        for (i, s) in bts.iter().enumerate() {
            words.push(s.to_string());
            ids.insert(s.to_string(), i);
        }
        for ss in synonyms {
            uf.union(ids[&ss[0]], ids[&ss[1]]);
        }
        let all_groups = uf.groups();
        let mut groups: Vec<Vec<usize>> = vec![];
        for s in text.split_whitespace() {
            let wid = ids[s];
            let gid = uf.find(wid);
            let group = all_groups[&gid].to_vec();
            groups.push(group);
        }
        let m = groups.len();
        let mut res = vec![];
        let mut cur = vec![];
        Self::dfs(0, &mut cur, &mut res, &groups, &words, m);
        res
    }

    fn dfs(
        start: usize,
        cur: &mut Vec<usize>,
        all: &mut Vec<String>,
        groups: &[Vec<usize>],
        words: &[String],
        m: usize,
    ) {
        if start == m {
            let ss: Vec<String> = cur.iter().map(|&i| words[i].to_string()).collect();
            all.push(ss.join(" "));
        } else {
            for &i in &groups[start] {
                cur.push(i);
                Self::dfs(start + 1, cur, all, groups, words, m);
                cur.pop();
            }
        }
    }
}

#[test]
fn test() {
    let synonyms = vec_vec_string![["happy", "joy"], ["sad", "sorrow"], ["joy", "cheerful"]];
    let text = "I am happy today but was sad yesterday".to_string();
    let res: Vec<String> = vec_string![
        "I am cheerful today but was sad yesterday",
        "I am cheerful today but was sorrow yesterday",
        "I am happy today but was sad yesterday",
        "I am happy today but was sorrow yesterday",
        "I am joy today but was sad yesterday",
        "I am joy today but was sorrow yesterday"
    ];
    assert_eq!(Solution::generate_sentences(synonyms, text), res);
}
