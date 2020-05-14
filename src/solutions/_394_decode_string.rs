pub struct Solution;

impl Solution {
    pub fn decode(s: &[char], m: usize, i: &mut usize) -> String {
        let mut res = "".to_string();
        while *i < m && s[*i] != ']' {
            if s[*i].is_digit(10) {
                let mut n = 0;
                while *i < m && s[*i].is_digit(10) {
                    n *= 10;
                    n += (s[*i] as u8 - b'0') as i32;
                    *i += 1;
                }
                *i += 1;
                let t = Self::decode(s, m, i);
                *i += 1;
                for _ in 0..n {
                    res += &t;
                }
            } else {
                res.push(s[*i]);
                *i += 1;
            }
        }
        res
    }
    pub fn decode_string(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let mut i = 0;
        Self::decode(&s, s.len(), &mut i)
    }
}

#[test]
fn test() {
    let s = "3[a]2[bc]".to_string();
    let res = "aaabcbc".to_string();
    assert_eq!(Solution::decode_string(s), res);
}
