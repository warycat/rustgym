struct Solution;

impl Solution {
    fn score_of_parentheses(s: String) -> i32 {
        let mut stack: Vec<i32> = vec![];
        for c in s.chars() {
            if c == '(' {
                stack.push(0);
            } else {
                let mut sum = 0;
                while let Some(last) = stack.pop() {
                    if last != 0 {
                        sum += last;
                    } else {
                        break;
                    }
                }
                if sum == 0 {
                    stack.push(1);
                } else {
                    stack.push(2 * sum);
                }
            }
        }
        stack.iter().sum()
    }
}

#[test]
fn test() {
    let s = "()".to_string();
    let res = 1;
    assert_eq!(Solution::score_of_parentheses(s), res);
    let s = "(())".to_string();
    let res = 2;
    assert_eq!(Solution::score_of_parentheses(s), res);
    let s = "()()".to_string();
    let res = 2;
    assert_eq!(Solution::score_of_parentheses(s), res);
    let s = "(()(()))".to_string();
    let res = 6;
    assert_eq!(Solution::score_of_parentheses(s), res);
}
