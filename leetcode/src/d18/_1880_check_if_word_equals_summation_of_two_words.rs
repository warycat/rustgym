struct Solution;

trait NumVal {
    fn val(&self) -> i32;
}

impl NumVal for String {
    fn val(&self) -> i32 {
        let mut res = 0;
        for b in self.bytes() {
            let x = b - b'a';
            res *= 10;
            res += x as i32;
        }
        res
    }
}

impl Solution {
    fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        first_word.val() + second_word.val() == target_word.val()
    }
}

#[test]
fn test() {
    let first_word = "acb".to_string();
    let second_word = "cba".to_string();
    let target_word = "cdb".to_string();
    let res = true;
    assert_eq!(
        Solution::is_sum_equal(first_word, second_word, target_word),
        res
    );
    let first_word = "aaa".to_string();
    let second_word = "a".to_string();
    let target_word = "aab".to_string();
    let res = false;
    assert_eq!(
        Solution::is_sum_equal(first_word, second_word, target_word),
        res
    );
    let first_word = "aaa".to_string();
    let second_word = "a".to_string();
    let target_word = "aaaa".to_string();
    let res = true;
    assert_eq!(
        Solution::is_sum_equal(first_word, second_word, target_word),
        res
    );
}
