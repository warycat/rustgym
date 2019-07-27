struct Solution;

impl Solution {
    fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' | ']' | '}' => match stack.pop() {
                    Some(t) => {
                        if !((t == '{' && c == '}')
                            || (t == '(' && c == ')')
                            || (t == '[' && c == ']'))
                        {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                },
                _ => {}
            }
        }
        stack.is_empty()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_valid(String::from("()")), true);
    assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
    assert_eq!(Solution::is_valid(String::from("(]")), false);
    assert_eq!(Solution::is_valid(String::from("([)]")), false);
    assert_eq!(Solution::is_valid(String::from("{[]}")), true);
}
