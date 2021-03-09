struct Solution;

impl Solution {
    fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        for (i, word) in sentence.split_whitespace().enumerate() {
            if word.starts_with(&search_word) {
                return (i + 1) as i32;
            }
        }
        -1
    }
}

#[test]
fn test() {
    let sentence = "i love eating burger".to_string();
    let search_word = "burg".to_string();
    let res = 4;
    assert_eq!(Solution::is_prefix_of_word(sentence, search_word), res);
    let sentence = "this problem is an easy problem".to_string();
    let search_word = "pro".to_string();
    let res = 2;
    assert_eq!(Solution::is_prefix_of_word(sentence, search_word), res);
    let sentence = "i am tired".to_string();
    let search_word = "you".to_string();
    let res = -1;
    assert_eq!(Solution::is_prefix_of_word(sentence, search_word), res);
    let sentence = "i use triple pillow".to_string();
    let search_word = "pill".to_string();
    let res = 4;
    assert_eq!(Solution::is_prefix_of_word(sentence, search_word), res);
    let sentence = "hello from the other side".to_string();
    let search_word = "they".to_string();
    let res = -1;
    assert_eq!(Solution::is_prefix_of_word(sentence, search_word), res);
}
