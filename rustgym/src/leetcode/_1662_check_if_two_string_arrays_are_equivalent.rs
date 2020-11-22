struct Solution;

impl Solution {
    fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let s1 = word1.join("");
        let s2 = word2.join("");
        s1 == s2
    }
}

#[test]
fn test() {
    let word1 = vec_string!["ab", "c"];
    let word2 = vec_string!["a", "bc"];
    let res = true;
    assert_eq!(Solution::array_strings_are_equal(word1, word2), res);
    let word1 = vec_string!["a", "cb"];
    let word2 = vec_string!["ab", "c"];
    let res = false;
    assert_eq!(Solution::array_strings_are_equal(word1, word2), res);
    let word1 = vec_string!["abc", "d", "defg"];
    let word2 = vec_string!["abcddefg"];
    let res = true;
    assert_eq!(Solution::array_strings_are_equal(word1, word2), res);
}
