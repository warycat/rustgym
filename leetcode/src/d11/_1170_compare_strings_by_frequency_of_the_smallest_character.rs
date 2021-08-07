struct Solution;

impl Solution {
    fn f(s: &str) -> usize {
        let mut count = vec![0; 26];
        let mut min = b'z';
        for b in s.bytes() {
            min = min.min(b);
            count[(b - b'a') as usize] += 1;
        }
        count[(min - b'a') as usize]
    }
    fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let f_words: Vec<usize> = words.iter().map(|s| Self::f(s)).collect();
        let mut counts = vec![0; 12];
        for f in f_words {
            counts[f] += 1;
        }
        for i in (1..10).rev() {
            counts[i] += counts[i + 1];
        }
        queries
            .iter()
            .map(|s| Self::f(s))
            .map(|f| counts[f + 1])
            .collect()
    }
}

#[test]
fn test() {
    let queries = vec_string!["cbd"];
    let words = vec_string!["zaaaz"];
    let res = vec![1];
    assert_eq!(Solution::num_smaller_by_frequency(queries, words), res);
    let queries = vec_string!["bbb", "cc"];
    let words = vec_string!["a", "aa", "aaa", "aaaa"];
    let res = vec![1, 2];
    assert_eq!(Solution::num_smaller_by_frequency(queries, words), res);
}
