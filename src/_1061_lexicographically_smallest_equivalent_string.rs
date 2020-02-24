struct Solution;

struct UnionFind {
    parents: Vec<usize>,
}

impl UnionFind {
    fn new() -> Self {
        let parents = (0..26).collect();
        UnionFind { parents }
    }
    fn find(&mut self, i: usize) -> usize {
        let j = self.parents[i];
        if i == j {
            j
        } else {
            self.parents[i] = self.find(j);
            self.parents[i]
        }
    }

    fn merge(&mut self, mut i: usize, mut j: usize) {
        i = self.find(i);
        j = self.find(j);
        if i != j {
            let min = i.min(j);
            self.parents[i] = min;
            self.parents[j] = min;
        }
    }
}

impl Solution {
    fn smallest_equivalent_string(a: String, b: String, s: String) -> String {
        let a: Vec<usize> = a.bytes().map(|b| (b - b'a') as usize).collect();
        let b: Vec<usize> = b.bytes().map(|b| (b - b'a') as usize).collect();
        let n = a.len();
        let mut uf = UnionFind::new();
        for i in 0..n {
            uf.merge(a[i], b[i]);
        }
        s.bytes()
            .map(|c| (uf.find((c - b'a') as usize) as u8 + b'a') as char)
            .collect()
    }
}

#[test]
fn test() {
    let a = "parker".to_string();
    let b = "morris".to_string();
    let s = "parser".to_string();
    let res = "makkek".to_string();
    assert_eq!(Solution::smallest_equivalent_string(a, b, s), res);
    let a = "hello".to_string();
    let b = "world".to_string();
    let s = "hold".to_string();
    let res = "hdld".to_string();
    assert_eq!(Solution::smallest_equivalent_string(a, b, s), res);
    let a = "leetcode".to_string();
    let b = "programs".to_string();
    let s = "sourcecode".to_string();
    let res = "aauaaaaada".to_string();
    assert_eq!(Solution::smallest_equivalent_string(a, b, s), res);
}
