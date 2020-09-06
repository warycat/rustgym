struct Solution;

impl Solution {
    fn modify_string(s: String) -> String {
        let mut s: Vec<char> = s.chars().collect();
        let n = s.len();
        for i in 0..n {
            if s[i] == '?' {
                s[i] = 'a';
                while (i > 0 && s[i] == s[i - 1]) || (i + 1 < n && s[i] == s[i + 1]) {
                    s[i] = (s[i] as u8 + 1) as char;
                }
            }
        }
        s.into_iter().collect()
    }
}

#[test]
fn test() {
    let s = "?zs".to_string();
    let res = "azs".to_string();
    assert_eq!(Solution::modify_string(s), res);
    let s = "ubv?w".to_string();
    let res = "ubvaw".to_string();
    assert_eq!(Solution::modify_string(s), res);
    let s = "j?qg??b".to_string();
    let res = "jaqgacb".to_string();
    assert_eq!(Solution::modify_string(s), res);
    let s = "??yw?ipkj?".to_string();
    let res = "abywaipkja".to_string();
    assert_eq!(Solution::modify_string(s), res);
}
