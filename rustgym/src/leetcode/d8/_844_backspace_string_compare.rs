struct Solution;

impl Solution {
    fn backspace_compare(s: String, t: String) -> bool {
        let mut ss: Vec<char> = vec![];
        let mut ts: Vec<char> = vec![];
        for c in s.chars() {
            if c == '#' {
                ss.pop();
            } else {
                ss.push(c);
            }
        }
        for c in t.chars() {
            if c == '#' {
                ts.pop();
            } else {
                ts.push(c);
            }
        }
        ss == ts
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string()),
        true
    );
    assert_eq!(
        Solution::backspace_compare("ab##".to_string(), "c#d#".to_string()),
        true
    );
    assert_eq!(
        Solution::backspace_compare("a##c".to_string(), "#a#c".to_string()),
        true
    );
    assert_eq!(
        Solution::backspace_compare("a#c".to_string(), "b".to_string()),
        false
    );
}
