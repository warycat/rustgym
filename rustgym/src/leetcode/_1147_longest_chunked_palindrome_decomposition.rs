struct Solution;

impl Solution {
    fn longest_decomposition(text: String) -> i32 {
        Self::greedy(&text)
    }

    fn greedy(s: &str) -> i32 {
        let n = s.len();
        if n == 0 {
            return 0;
        }
        for i in 1..=n / 2 {
            if s[0..i] == s[n - i..] {
                return 2 + Self::greedy(&s[i..n - i]);
            }
        }
        1
    }
}

#[test]
fn test() {
    let text = "ghiabcdefhelloadamhelloabcdefghi".to_string();
    let res = 7;
    assert_eq!(Solution::longest_decomposition(text), res);
    let text = "merchant".to_string();
    let res = 1;
    assert_eq!(Solution::longest_decomposition(text), res);
    let text = "antaprezatepzapreanta".to_string();
    let res = 11;
    assert_eq!(Solution::longest_decomposition(text), res);
    let text = "aaa".to_string();
    let res = 3;
    assert_eq!(Solution::longest_decomposition(text), res);
    let text = "elvtoelvto".to_string();
    let res = 2;
    assert_eq!(Solution::longest_decomposition(text), res);
}
