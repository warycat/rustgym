struct Solution;

impl Solution {
    fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let mut t = "".to_string();
        for w in words {
            t.push_str(&w);
            if t == s {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    let s = "iloveleetcode".to_string();
    let words = vec_string!["i", "love", "leetcode", "apples"];
    let res = true;
    assert_eq!(Solution::is_prefix_string(s, words), res);
    let s = "iloveleetcode".to_string();
    let words = vec_string!["apples", "i", "love", "leetcode"];
    let res = false;
    assert_eq!(Solution::is_prefix_string(s, words), res);
}
