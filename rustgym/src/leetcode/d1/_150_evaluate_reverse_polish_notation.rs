struct Solution;

impl Solution {
    fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        for tok in tokens {
            match tok.as_ref() {
                "+" => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left + right);
                }
                "-" => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left - right);
                }
                "*" => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left * right);
                }
                "/" => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left / right);
                }
                _ => {
                    stack.push(tok.parse::<i32>().unwrap());
                }
            }
        }
        stack[0]
    }
}

#[test]
fn test() {
    let tokens = vec_string!["2", "1", "+", "3", "*"];
    let res = 9;
    assert_eq!(Solution::eval_rpn(tokens), res);
    let tokens = vec_string!["4", "13", "5", "/", "+"];
    let res = 6;
    assert_eq!(Solution::eval_rpn(tokens), res);
    let tokens = vec_string!["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"];
    let res = 22;
    assert_eq!(Solution::eval_rpn(tokens), res);
}
