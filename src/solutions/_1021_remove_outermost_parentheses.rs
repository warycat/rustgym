struct Solution;

impl Solution {
    fn remove_outer_parentheses(s: String) -> String {
        let mut res: String = "".to_string();
        let mut count = 0;
        for c in s.chars() {
            if c == '(' {
                count += 1;
                if count > 1 {
                    res.push(c);
                }
            } else {
                count -= 1;
                if count != 0 {
                    res.push(c);
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "(()())(())".to_string();
    let t = "()()()".to_string();
    assert_eq!(Solution::remove_outer_parentheses(s), t);
    let s = "(()())(())(()(()))".to_string();
    let t = "()()()()(())".to_string();
    assert_eq!(Solution::remove_outer_parentheses(s), t);
    let s = "()()".to_string();
    let t = "".to_string();
    assert_eq!(Solution::remove_outer_parentheses(s), t);
}
