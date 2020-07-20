struct Solution;

impl Solution {
    fn decode_at_index(s: String, k: i32) -> String {
        let mut k = k as u64;
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut size = 0;
        for i in 0..n {
            match s[i] {
                '0'..='9' => {
                    size *= (s[i] as u8 - b'0') as u64;
                }
                _ => {
                    size += 1;
                }
            }
        }
        let mut res = "".to_string();
        for i in (0..n).rev() {
            k %= size;
            match s[i] {
                '0'..='9' => {
                    size /= (s[i] as u8 - b'0') as u64;
                }
                _ => {
                    if k == 0 {
                        res = s[i].to_string();
                        break;
                    } else {
                        size -= 1;
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "leet2code3".to_string();
    let k = 10;
    let res = "o".to_string();
    assert_eq!(Solution::decode_at_index(s, k), res);
}
