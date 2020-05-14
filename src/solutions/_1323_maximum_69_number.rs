struct Solution;

impl Solution {
    fn maximum69_number(mut num: i32) -> i32 {
        let mut stack: Vec<i32> = vec![];
        while num > 0 {
            stack.push(num % 10);
            num /= 10;
        }
        let n = stack.len();
        let mut changed = false;
        let mut res = 0;
        for i in (0..n).rev() {
            if stack[i] == 6 && !changed {
                res = res * 10 + 9;
                changed = true;
            } else {
                res = res * 10 + stack[i];
            }
        }
        res
    }
}

#[test]
fn test() {
    let num = 9669;
    let res = 9969;
    assert_eq!(Solution::maximum69_number(num), res);
    let num = 9996;
    let res = 9999;
    assert_eq!(Solution::maximum69_number(num), res);
    let num = 9999;
    let res = 9999;
    assert_eq!(Solution::maximum69_number(num), res);
}
