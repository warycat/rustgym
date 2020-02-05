struct Solution;

impl Solution {
    fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        let words: Vec<&str> = text.split_whitespace().collect();
        words.windows(3).for_each(|v| {
            if v[0] == first && v[1] == second {
                res.push(v[2].to_string());
            }
        });
        res
    }
}

#[test]
fn test() {
    let text = "alice is a good girl she is a good student".to_string();
    let first = "a".to_string();
    let second = "good".to_string();
    let res: Vec<String> = vec_string!["girl", "student"];
    assert_eq!(Solution::find_ocurrences(text, first, second), res);
    let text = "we will we will rock you".to_string();
    let first = "we".to_string();
    let second = "will".to_string();
    let res: Vec<String> = vec_string!["we", "rock"];
    assert_eq!(Solution::find_ocurrences(text, first, second), res);
}
