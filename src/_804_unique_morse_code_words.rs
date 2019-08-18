struct Solution;

use std::collections::HashSet;

impl Solution {
    fn unique_morse_representations(words: Vec<String>) -> i32 {
        let map = [
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];
        let mut morse: HashSet<String> = HashSet::new();
        for w in words {
            let mut s: String = "".to_string();
            for c in w.chars() {
                let m = map[(c as u8 - 'a' as u8) as usize];
                s += m;
            }
            morse.insert(s);
        }
        morse.len() as i32
    }
}

#[test]
fn test() {
    let words: Vec<String> = ["gin", "zen", "gig", "msg"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(Solution::unique_morse_representations(words), 2);
}
