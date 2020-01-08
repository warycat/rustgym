struct Solution;

impl Solution {
    fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut mapping: Vec<char> = vec![0 as char; 256];
        for (i, c) in order.chars().enumerate() {
            mapping[c as usize] = (i as u8 + b'a') as char;
        }
        let words: Vec<String> = words
            .into_iter()
            .map(|s| Solution::translate(s, &mapping))
            .collect();
        let mut sorted: Vec<String> = words.to_vec();
        sorted.sort();
        words == sorted
    }

    fn translate(s: String, mapping: &[char]) -> String {
        s.chars().map(|c| mapping[c as usize]).collect()
    }
}

#[test]
fn test() {
    let words: Vec<String> = vec!["hello", "leetcode"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let order: String = "hlabcdefgijkmnopqrstuvwxyz".to_string();
    assert_eq!(Solution::is_alien_sorted(words, order), true);

    let words: Vec<String> = vec!["word", "world", "row"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let order: String = "worldabcefghijkmnpqstuvxyz".to_string();
    assert_eq!(Solution::is_alien_sorted(words, order), false);

    let words: Vec<String> = vec!["apple", "app"].iter().map(|s| s.to_string()).collect();
    let order: String = "abcdefghijklmnopqrstuvwxyz".to_string();
    assert_eq!(Solution::is_alien_sorted(words, order), false);

    let words: Vec<String> = vec!["kuvp", "q"].iter().map(|s| s.to_string()).collect();
    let order: String = "ngxlkthsjuoqcpavbfdermiywz".to_string();
    assert_eq!(Solution::is_alien_sorted(words, order), true);
}
