struct Solution;

impl Solution {
    fn is_one_edit_distance(s: String, t: String) -> bool {
        let (short, long): (Vec<char>, Vec<char>) = if s.len() < t.len() {
            (s.chars().collect(), t.chars().collect())
        } else {
            (t.chars().collect(), s.chars().collect())
        };
        let n = short.len();
        let m = long.len();
        if m - n > 1 {
            return false;
        }
        let mut i = 0;
        while i < n {
            if short[i] == long[i] {
                i += 1;
            } else {
                if n == m {
                    return short[i + 1..n] == long[i + 1..m];
                } else {
                    return short[i..n] == long[i + 1..m];
                }
            }
        }
        n != m
    }
}

#[test]
fn test() {
    let s = "".to_string();
    let t = "".to_string();
    let res = false;
    assert_eq!(Solution::is_one_edit_distance(s, t), res);
    let s = "ab".to_string();
    let t = "acb".to_string();
    let res = true;
    assert_eq!(Solution::is_one_edit_distance(s, t), res);
    let s = "cab".to_string();
    let t = "ad".to_string();
    let res = false;
    assert_eq!(Solution::is_one_edit_distance(s, t), res);
    let s = "1203".to_string();
    let t = "1213".to_string();
    let res = true;
    assert_eq!(Solution::is_one_edit_distance(s, t), res);
    let s = "c".to_string();
    let t = "c".to_string();
    let res = false;
    assert_eq!(Solution::is_one_edit_distance(s, t), res);
    let s = "a".to_string();
    let t = "".to_string();
    let res = true;
    assert_eq!(Solution::is_one_edit_distance(s, t), res);
}
