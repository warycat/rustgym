struct Solution;

impl Solution {
    fn min_operations(s: String) -> i32 {
        let mut odd = [0, 0];
        let mut even = [0, 0];
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        for i in 0..n {
            if i % 2 == 0 {
                if s[i] == '0' {
                    even[0] += 1;
                } else {
                    even[1] += 1;
                }
            } else {
                if s[i] == '0' {
                    odd[0] += 1;
                } else {
                    odd[1] += 1;
                }
            }
        }
        let a = (even[0] - (n + 1) as i32 / 2).abs() + (odd[1] - n as i32 / 2).abs();
        let b = (odd[0] - n as i32 / 2).abs() + (even[1] - (n + 1) as i32 / 2).abs();
        a.min(b)
    }
}

#[test]
fn test() {
    let s = "0100".to_string();
    let res = 1;
    assert_eq!(Solution::min_operations(s), res);
    let s = "10".to_string();
    let res = 0;
    assert_eq!(Solution::min_operations(s), res);
    let s = "1111".to_string();
    let res = 2;
    assert_eq!(Solution::min_operations(s), res);
}
