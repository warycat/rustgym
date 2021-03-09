struct Solution;

impl Solution {
    fn min_add_to_make_valid(s: String) -> i32 {
        let mut stack: Vec<char> = vec![];
        let mut res = 0;
        for c in s.chars() {
            match c {
                '(' => {
                    stack.push(c);
                }
                ')' => {
                    if stack.is_empty() {
                        res += 1;
                    } else {
                        stack.pop();
                    }
                }
                _ => {}
            }
        }
        res + stack.len() as i32
    }
}

#[test]
fn test() {
    let s = "())".to_string();
    let res = 1;
    assert_eq!(Solution::min_add_to_make_valid(s), res);
    let s = "(((".to_string();
    let res = 3;
    assert_eq!(Solution::min_add_to_make_valid(s), res);
    let s = "()".to_string();
    let res = 0;
    assert_eq!(Solution::min_add_to_make_valid(s), res);
    let s = "()))((".to_string();
    let res = 4;
    assert_eq!(Solution::min_add_to_make_valid(s), res);
}
