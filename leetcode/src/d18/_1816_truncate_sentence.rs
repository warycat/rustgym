struct Solution;

impl Solution {
    fn truncate_sentence(s: String, k: i32) -> String {
        let words: Vec<&str> = s.split_whitespace().take(k as usize).collect();
        words.join(" ")
    }
}

#[test]
fn test() {
    let s = "Hello how are you Contestant".to_string();
    let k = 4;
    let res = "Hello how are you".to_string();
    assert_eq!(Solution::truncate_sentence(s, k), res);
}
