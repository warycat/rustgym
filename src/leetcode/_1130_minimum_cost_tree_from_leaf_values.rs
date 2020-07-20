struct Solution;
use std::i32;

impl Solution {
    fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut stack: Vec<i32> = vec![i32::MAX];
        for x in arr {
            while x >= *stack.last().unwrap() {
                let mid = stack.pop().unwrap();
                let y = *stack.last().unwrap();
                res += mid * i32::min(y, x);
            }
            stack.push(x);
        }
        while stack.len() > 2 {
            let x = stack.pop().unwrap();
            let y = *stack.last().unwrap();
            res += x * y;
        }
        res
    }
}

#[test]
fn test() {
    let arr = vec![6, 2, 4];
    assert_eq!(Solution::mct_from_leaf_values(arr), 32);
}
