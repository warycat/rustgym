struct Solution;

impl Solution {
    fn sort_string(s: String) -> String {
        let mut count: Vec<usize> = vec![0; 26];
        let mut n = s.len();
        for c in s.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }
        let mut direction = true;
        let mut res = "".to_string();
        while n > 0 {
            if direction {
                for i in 0..26 {
                    if count[i] > 0 {
                        count[i] -= 1;
                        n -= 1;
                        res.push((b'a' + i as u8) as char);
                    }
                }
            } else {
                for i in (0..26).rev() {
                    if count[i] > 0 {
                        count[i] -= 1;
                        n -= 1;
                        res.push((b'a' + i as u8) as char);
                    }
                }
            }
            direction = !direction;
        }
        res
    }
}

#[test]
fn test() {
    let s = "aaaabbbbcccc".to_string();
    let res = "abccbaabccba".to_string();
    assert_eq!(Solution::sort_string(s), res);
    let s = "rat".to_string();
    let res = "art".to_string();
    assert_eq!(Solution::sort_string(s), res);
    let s = "leetcode".to_string();
    let res = "cdelotee".to_string();
    assert_eq!(Solution::sort_string(s), res);
    let s = "ggggggg".to_string();
    let res = "ggggggg".to_string();
    assert_eq!(Solution::sort_string(s), res);
    let s = "spo".to_string();
    let res = "ops".to_string();
    assert_eq!(Solution::sort_string(s), res);
}
