struct Solution;

#[derive(Debug)]
enum Tok {
    Left,
    Pair(i32),
}

impl Solution {
    fn longest_valid_parentheses(s: String) -> i32 {
        let mut res = 0;
        let mut stack: Vec<Tok> = vec![];
        for c in s.chars() {
            if c == '(' {
                stack.push(Tok::Left)
            } else {
                match stack.pop() {
                    Some(Tok::Left) => {
                        if let Some(Tok::Pair(x)) = stack.last_mut() {
                            *x += 2;
                            res = res.max(*x);
                        } else {
                            stack.push(Tok::Pair(2));
                            res = res.max(2);
                        }
                    }
                    Some(Tok::Pair(x)) => {
                        if let Some(Tok::Left) = stack.pop() {
                            if let Some(Tok::Pair(y)) = stack.last_mut() {
                                *y += x + 2;
                                res = res.max(*y);
                            } else {
                                stack.push(Tok::Pair(x + 2));
                                res = res.max(x + 2);
                            }
                        }
                    }
                    None => {}
                }
            }
        }
        if let Some(Tok::Pair(x)) = stack.pop() {
            res = res.max(x);
        }
        res
    }
}

#[test]
fn test() {
    let s = "(()".to_string();
    let res = 2;
    assert_eq!(Solution::longest_valid_parentheses(s), res);
    let s = ")()())".to_string();
    let res = 4;
    assert_eq!(Solution::longest_valid_parentheses(s), res);
    let s = "()(())".to_string();
    let res = 6;
    assert_eq!(Solution::longest_valid_parentheses(s), res);
}
