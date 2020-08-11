struct Solution;

impl Solution {
    fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut mapping: Vec<char> = vec![0 as char; 256];
        for (i, c) in order.chars().enumerate() {
            mapping[c as usize] = (i as u8 + b'a') as char;
        }
        let words: Vec<String> = words
            .into_iter()
            .map(|s| Self::translate(s, &mapping))
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
    let words: Vec<String> = vec_string!["hello", "leetcode"];
    let order: String = "hlabcdefgijkmnopqrstuvwxyz".to_string();
    assert_eq!(Solution::is_alien_sorted(words, order), true);

    let words: Vec<String> = vec_string!["word", "world", "row"];
    let order: String = "worldabcefghijkmnpqstuvxyz".to_string();
    assert_eq!(Solution::is_alien_sorted(words, order), false);

    let words: Vec<String> = vec_string!["apple", "app"];
    let order: String = "abcdefghijklmnopqrstuvwxyz".to_string();
    assert_eq!(Solution::is_alien_sorted(words, order), false);

    let words: Vec<String> = vec_string!["kuvp", "q"];
    let order: String = "ngxlkthsjuoqcpavbfdermiywz".to_string();
    assert_eq!(Solution::is_alien_sorted(words, order), true);
}
