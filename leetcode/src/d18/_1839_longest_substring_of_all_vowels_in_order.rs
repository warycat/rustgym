struct Solution;

impl Solution {
    fn longest_beautiful_substring(word: String) -> i32 {
        let word: Vec<char> = word.chars().collect();
        let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
        let mut start = 0;
        let n = word.len();
        let mut res = 0;
        'outer: while start < n {
            while start < n && word[start] != 'a' {
                start += 1;
            }
            let mut end = start;
            for i in 0..5 {
                if !(end < n && word[end] == vowels[i]) {
                    start = end;
                    continue 'outer;
                }
                while end < n && word[end] == vowels[i] {
                    end += 1;
                }
            }
            res = res.max(end - start);
            start = end;
        }
        res as i32
    }
}

#[test]
fn test() {
    let word = "aeiaaioaaaaeiiiiouuuooaauuaeiu".to_string();
    let res = 13;
    assert_eq!(Solution::longest_beautiful_substring(word), res);
    let word = "aeeeiiiioooauuuaeiou".to_string();
    let res = 5;
    assert_eq!(Solution::longest_beautiful_substring(word), res);
    let word = "a".to_string();
    let res = 0;
    assert_eq!(Solution::longest_beautiful_substring(word), res);
}
