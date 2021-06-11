struct Solution;

impl Solution {
    fn replace_digits(s: String) -> String {
        let mut s: Vec<char> = s.chars().collect();
        let n = s.len();
        for i in 0..n {
            if i % 2 == 1 {
                let x = s[i] as u8 - b'0';
                let c = s[i - 1] as u8;
                let d = (c + x) as char;
                s[i] = d;
            }
        }
        s.into_iter().collect()
    }
}

#[test]
fn test() {
    let s = "a1c1e1".to_string();
    let res = "abcdef".to_string();
    assert_eq!(Solution::replace_digits(s), res);
    let s = "a1b2c3d4e".to_string();
    let res = "abbdcfdhe".to_string();
    assert_eq!(Solution::replace_digits(s), res);
}
