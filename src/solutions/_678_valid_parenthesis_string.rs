struct Solution;

impl Solution {
    fn check_valid_string(s: String) -> bool {
        let mut lo = 0;
        let mut hi = 0;
        for c in s.chars() {
            match c {
                '(' => {
                    lo += 1;
                    hi += 1;
                }
                ')' => {
                    lo = 0.max(lo - 1);
                    hi -= 1;
                }
                _ => {
                    lo = 0.max(lo - 1);
                    hi += 1;
                }
            }
            if hi < 0 {
                return false;
            }
        }
        lo == 0
    }
}

#[test]
fn test() {
    let s = "()".to_string();
    let res = true;
    assert_eq!(Solution::check_valid_string(s), res);
    let s = "(*)".to_string();
    let res = true;
    assert_eq!(Solution::check_valid_string(s), res);
    let s = "(*))".to_string();
    let res = true;
    assert_eq!(Solution::check_valid_string(s), res);
}
