struct Solution;

impl Solution {
    fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for c in s.chars() {
            let n = stack.len();
            if n > 1 && c == 'c' && stack[n - 1] == 'b' && stack[n - 2] == 'a' {
                stack.pop();
                stack.pop();
            } else {
                stack.push(c);
            }
        }
        stack.is_empty()
    }
}

#[test]
fn test() {
    let s = "aabcbc".to_string();
    let res = true;
    assert_eq!(Solution::is_valid(s), res);
    let s = "abcabcababcc".to_string();
    let res = true;
    assert_eq!(Solution::is_valid(s), res);
    let s = "abccba".to_string();
    let res = false;
    assert_eq!(Solution::is_valid(s), res);
    let s = "cababc".to_string();
    let res = false;
    assert_eq!(Solution::is_valid(s), res);
}
