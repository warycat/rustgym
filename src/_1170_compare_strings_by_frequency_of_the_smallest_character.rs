struct Solution;

use std::collections::BTreeMap;

impl Solution {
    fn f(s: &str) -> i32 {
        let mut a: BTreeMap<char, i32> = BTreeMap::new();
        for c in s.chars() {
            *a.entry(c).or_default() += 1;
        }
        *a.values().next().unwrap()
    }
    fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let queries: Vec<i32> = queries.iter().map(|s| Self::f(s)).collect();
        let mut words: Vec<i32> = words.iter().map(|s| Self::f(s)).collect();
        let n = words.len();
        words.sort_unstable();
        for q in queries {
            match words.binary_search(&q) {
                Ok(mut i) => {
                    while i + 1 < n && words[i + 1] == q {
                        i += 1;
                    }
                    res.push(n as i32 - (i + 1) as i32)
                }
                Err(i) => res.push(n as i32 - i as i32),
            }
        }
        res
    }
}

#[test]
fn test() {
    let queries = ["cbd"].iter().map(|s| s.to_string()).collect();
    let words = ["zaaaz"].iter().map(|s| s.to_string()).collect();
    let res = vec![1];
    assert_eq!(Solution::num_smaller_by_frequency(queries, words), res);
    let queries = ["bbb", "cc"].iter().map(|s| s.to_string()).collect();
    let words = ["a", "aa", "aaa", "aaaa"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let res = vec![1, 2];
    assert_eq!(Solution::num_smaller_by_frequency(queries, words), res);
}
