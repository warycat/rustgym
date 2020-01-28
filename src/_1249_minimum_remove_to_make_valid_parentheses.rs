struct Solution;

use std::collections::HashSet;

impl Solution {
    fn min_remove_to_make_valid(s: String) -> String {
        let mut stack: Vec<usize> = vec![];
        let mut res: String = "".to_string();
        let mut remove: HashSet<usize> = HashSet::new();
        for (i, c) in s.char_indices() {
            match c {
                '(' => {
                    stack.push(i);
                }
                ')' => {
                    if stack.pop().is_none() {
                        remove.insert(i);
                    }
                }
                _ => {}
            }
        }
        for i in stack {
            remove.insert(i);
        }
        for (i, c) in s.char_indices() {
            if !remove.contains(&i) {
                res.push(c);
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "lee(t(c)o)de)".to_string();
    let res = "lee(t(c)o)de".to_string();
    assert_eq!(Solution::min_remove_to_make_valid(s), res);
    let s = "a)b(c)d".to_string();
    let res = "ab(c)d".to_string();
    assert_eq!(Solution::min_remove_to_make_valid(s), res);
    let s = "))((".to_string();
    let res = "".to_string();
    assert_eq!(Solution::min_remove_to_make_valid(s), res);
    let s = "(a(b(c)d)".to_string();
    let res = "a(b(c)d)".to_string();
    assert_eq!(Solution::min_remove_to_make_valid(s), res);
}
