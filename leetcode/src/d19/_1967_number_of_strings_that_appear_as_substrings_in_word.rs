struct Solution;

impl Solution {
    fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        let mut res = 0;
        for p in patterns {
            if word.contains(&p) {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let patterns = vec_string!["a", "abc", "bc", "d"];
    let word = "abc".to_string();
    let res = 3;
    assert_eq!(Solution::num_of_strings(patterns, word), res);
    let patterns = vec_string!["a", "b", "c"];
    let word = "aaaaabbbbb".to_string();
    let res = 2;
    assert_eq!(Solution::num_of_strings(patterns, word), res);
    let patterns = vec_string!["a", "a", "a"];
    let word = "ab".to_string();
    let res = 3;
    assert_eq!(Solution::num_of_strings(patterns, word), res);
}
