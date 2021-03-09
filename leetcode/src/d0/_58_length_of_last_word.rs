struct Solution;

impl Solution {
    fn length_of_last_word(s: String) -> i32 {
        if let Some(last) = s.split_whitespace().last() {
            last.len() as i32
        } else {
            0
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::length_of_last_word(String::from("Hello World")),
        5
    );
}
