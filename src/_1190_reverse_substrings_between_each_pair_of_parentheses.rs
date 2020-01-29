struct Solution;

use std::collections::VecDeque;

impl Solution {
    fn reverse_parentheses(s: String) -> String {
        let mut stack: Vec<char> = vec![];
        let mut queue: VecDeque<char> = VecDeque::new();
        for c in s.chars() {
            if c == ')' {
                while let Some(top) = stack.pop() {
                    if top == '(' {
                        while let Some(front) = queue.pop_front() {
                            stack.push(front);
                        }
                        break;
                    } else {
                        queue.push_back(top);
                    }
                }
            } else {
                stack.push(c);
            }
        }
        stack.iter().collect()
    }
}

#[test]
fn test() {
    let s = "(abcd)".to_string();
    let res = "dcba".to_string();
    assert_eq!(Solution::reverse_parentheses(s), res);
    let s = "(u(love)i)".to_string();
    let res = "iloveu".to_string();
    assert_eq!(Solution::reverse_parentheses(s), res);
    let s = "(ed(et(oc))el)".to_string();
    let res = "leetcode".to_string();
    assert_eq!(Solution::reverse_parentheses(s), res);
    let s = "a(bcdefghijkl(mno)p)q".to_string();
    let res = "apmnolkjihgfedcbq".to_string();
    assert_eq!(Solution::reverse_parentheses(s), res);
}
