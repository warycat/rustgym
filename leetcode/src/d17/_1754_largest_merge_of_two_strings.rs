struct Solution;

impl Solution {
    fn largest_merge(word1: String, word2: String) -> String {
        Solution::merge(&word1, &word2)
    }

    fn merge(w1: &str, w2: &str) -> String {
        if w1.is_empty() || w2.is_empty() {
            format!("{}{}", w1, w2)
        } else {
            if w1 > w2 {
                format!("{}{}", &w1[0..1], Solution::merge(&w1[1..], w2))
            } else {
                format!("{}{}", &w2[0..1], Solution::merge(w1, &w2[1..]))
            }
        }
    }
}

#[test]
fn test() {
    let word1 = "cabaa".to_string();
    let word2 = "bcaaa".to_string();
    let res = "cbcabaaaaa".to_string();
    assert_eq!(Solution::largest_merge(word1, word2), res);
    let word1 = "abcabc".to_string();
    let word2 = "abdcaba".to_string();
    let res = "abdcabcabcaba".to_string();
    assert_eq!(Solution::largest_merge(word1, word2), res);
}
