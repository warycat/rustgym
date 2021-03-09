struct Solution;

impl Solution {
    fn max_depth(s: String) -> i32 {
        let mut res = 0;
        let mut left = 0;
        for c in s.chars() {
            if c == '(' {
                left += 1;
            }
            if c == ')' {
                left -= 1;
            }
            res = res.max(left);
        }
        res
    }
}

#[test]
fn test() {
    let s = "(1+(2*3)+((8)/4))+1".to_string();
    let res = 3;
    assert_eq!(Solution::max_depth(s), res);
    let s = "(1)+((2))+(((3)))".to_string();
    let res = 3;
    assert_eq!(Solution::max_depth(s), res);
    let s = "1+(2*3)/(2-1)".to_string();
    let res = 1;
    assert_eq!(Solution::max_depth(s), res);
    let s = "1".to_string();
    let res = 0;
    assert_eq!(Solution::max_depth(s), res);
}
