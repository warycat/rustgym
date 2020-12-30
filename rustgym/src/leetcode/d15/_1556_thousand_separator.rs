struct Solution;

impl Solution {
    fn thousand_separator(mut n: i32) -> String {
        if n == 0 {
            return "0".to_string();
        }
        let mut stack = vec![];
        let mut count = 0;
        while n > 0 {
            if count % 3 == 0 && count > 0 {
                stack.push('.');
            }
            stack.push((b'0' + (n % 10) as u8) as char);
            n /= 10;
            count += 1;
        }
        stack.into_iter().rev().collect()
    }
}

#[test]
fn test() {
    let n = 987;
    let res = "987".to_string();
    assert_eq!(Solution::thousand_separator(n), res);
    let n = 1234;
    let res = "1.234".to_string();
    assert_eq!(Solution::thousand_separator(n), res);
    let n = 123456789;
    let res = "123.456.789".to_string();
    assert_eq!(Solution::thousand_separator(n), res);
    let n = 0;
    let res = "0".to_string();
    assert_eq!(Solution::thousand_separator(n), res);
}
