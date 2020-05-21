struct Solution;

impl Solution {
    fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = vec![];
        let mut it = popped.iter().peekable();
        for x in pushed {
            stack.push(x);
            while let (Some(&a), Some(&&b)) = (stack.last(), it.peek()) {
                if a == b {
                    stack.pop();
                    it.next();
                } else {
                    break;
                }
            }
        }
        stack.is_empty()
    }
}

#[test]
fn test() {
    let pushed = vec![1, 2, 3, 4, 5];
    let popped = vec![4, 5, 3, 2, 1];
    let res = true;
    assert_eq!(Solution::validate_stack_sequences(pushed, popped), res);
    let pushed = vec![1, 2, 3, 4, 5];
    let popped = vec![4, 3, 5, 1, 2];
    let res = false;
    assert_eq!(Solution::validate_stack_sequences(pushed, popped), res);
}
