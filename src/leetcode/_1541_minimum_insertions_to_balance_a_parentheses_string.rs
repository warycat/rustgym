struct Solution;

impl Solution {
    fn min_insertions(s: String) -> i32 {
        let mut right = 0;
        let mut res = 0;
        for c in s.chars().rev() {
            if c == ')' {
                right += 1;
            } else {
                if right >= 2 {
                    right -= 2;
                    if right % 2 == 1 {
                        right += 1;
                        res += 1;
                    }
                } else {
                    if right == 0 {
                        res += 2;
                    } else {
                        res += 1;
                        right -= 1;
                    }
                }
            }
        }
        res += right / 2 + (right % 2) * 2;
        res
    }
}

#[test]
fn test() {
    let s = "())".to_string();
    let res = 0;
    assert_eq!(Solution::min_insertions(s), res);
    let s = "))())(".to_string();
    let res = 3;
    assert_eq!(Solution::min_insertions(s), res);
    let s = "((((((".to_string();
    let res = 12;
    assert_eq!(Solution::min_insertions(s), res);
    let s = ")))))))".to_string();
    let res = 5;
    assert_eq!(Solution::min_insertions(s), res);
    let s = "(()))(()))()())))".to_string();
    let res = 4;
    assert_eq!(Solution::min_insertions(s), res);
}
