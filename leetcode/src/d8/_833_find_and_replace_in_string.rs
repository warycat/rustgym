struct Solution;

type Pair = (usize, usize);

impl Solution {
    fn find_replace_string(
        mut s: String,
        indexes: Vec<i32>,
        sources: Vec<String>,
        targets: Vec<String>,
    ) -> String {
        let n = indexes.len();
        let mut v: Vec<Pair> = vec![];
        for i in 0..n {
            v.push((indexes[i] as usize, i));
        }
        v.sort_unstable();
        for (i, j) in v.into_iter().rev() {
            let source = &sources[j];
            let m = source.len();
            let target = &targets[j];
            if i + m <= s.len() && &s[i..i + m] == source {
                s.replace_range(i..i + m, target);
            }
        }
        s
    }
}

#[test]
fn test() {
    let s = "abcd".to_string();
    let indexes = vec![0, 2];
    let sources = vec_string!["a", "cd"];
    let targets = vec_string!["eee", "ffff"];
    let res = "eeebffff".to_string();
    assert_eq!(
        Solution::find_replace_string(s, indexes, sources, targets),
        res
    );
    let s = "abcd".to_string();
    let indexes = vec![0, 2];
    let sources = vec_string!["ab", "ec"];
    let targets = vec_string!["eee", "ffff"];
    let res = "eeecd".to_string();
    assert_eq!(
        Solution::find_replace_string(s, indexes, sources, targets),
        res
    );
}
