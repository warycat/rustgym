struct Solution;

impl Solution {
    fn calculate_time(keyboard: String, word: String) -> i32 {
        let mut indexes: Vec<i32> = vec![0; 26];
        for (i, b) in keyboard.bytes().enumerate() {
            indexes[(b - b'a') as usize] = i as i32;
        }
        let mut prev = 0;
        let mut sum = 0;
        for b in word.bytes() {
            let index = indexes[(b - b'a') as usize];
            sum += (index - prev).abs();
            prev = index;
        }
        sum
    }
}

#[test]
fn test() {
    let keyboard = "abcdefghijklmnopqrstuvwxyz".to_string();
    let word = "cba".to_string();
    assert_eq!(Solution::calculate_time(keyboard, word), 4);
    let keyboard = "pqrstuvwxyzabcdefghijklmno".to_string();
    let word = "leetcode".to_string();
    assert_eq!(Solution::calculate_time(keyboard, word), 73);
}
