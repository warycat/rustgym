struct Solution;

impl Solution {
    fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let s: Vec<char> = s.chars().collect();
        let mut prev: Option<usize> = None;
        let n = s.len();
        let mut res: Vec<i32> = vec![n as i32; n];
        for i in 0..n {
            if s[i] == c {
                prev = Some(i);
            }
            if let Some(j) = prev {
                res[i] = i32::min(res[i], (i - j) as i32);
            }
        }
        prev = None;
        for i in (0..n).rev() {
            if s[i] == c {
                prev = Some(i);
            }
            if let Some(j) = prev {
                res[i] = i32::min(res[i], (j - i) as i32);
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "loveleetcode".to_string();
    let t = vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0];
    assert_eq!(Solution::shortest_to_char(s, 'e'), t);
}
