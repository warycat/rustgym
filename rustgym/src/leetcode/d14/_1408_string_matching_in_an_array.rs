struct Solution;

impl Solution {
    fn string_matching(words: Vec<String>) -> Vec<String> {
        let n = words.len();
        let mut res = vec![];
        for i in 0..n {
            let mut found = false;
            for j in 0..n {
                if !found && i != j && words[j].contains(&words[i]) {
                    found = true;
                }
            }
            if found {
                res.push(words[i].to_string());
            }
        }
        res
    }
}

#[test]
fn test() {
    let words = vec_string!["mass", "as", "hero", "superhero"];
    let res = vec_string!["as", "hero"];
    assert_eq!(Solution::string_matching(words), res);
    let words = vec_string!["leetcode", "et", "code"];
    let res = vec_string!["et", "code"];
    assert_eq!(Solution::string_matching(words), res);
    let words = vec_string!["blue", "green", "bu"];
    let res = vec_string![];
    assert_eq!(Solution::string_matching(words), res);
}
