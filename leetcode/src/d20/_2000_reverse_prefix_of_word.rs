struct Solution;

impl Solution {
    fn reverse_prefix(word: String, ch: char) -> String {
        let n = word.len();
        let a: Vec<char> = word.chars().collect();
        match word.find(ch) {
            Some(j) => {
                let mut res = "".to_string();
                for i in (0..=j).rev() {
                    res.push(a[i]);
                }
                for i in j + 1..n {
                    res.push(a[i]);
                }
                res
            }
            None => word,
        }
    }
}

#[test]
fn test() {
    let word = "abcdefd".to_string();
    let ch = 'd';
    let res = "dcbaefd".to_string();
    assert_eq!(Solution::reverse_prefix(word, ch), res);
    let word = "xyxzxe".to_string();
    let ch = 'z';
    let res = "zxyxxe".to_string();
    assert_eq!(Solution::reverse_prefix(word, ch), res);
    let word = "abcd".to_string();
    let ch = 'z';
    let res = "abcd".to_string();
    assert_eq!(Solution::reverse_prefix(word, ch), res);
}
