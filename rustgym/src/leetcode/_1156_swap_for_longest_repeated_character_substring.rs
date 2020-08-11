struct Solution;

impl Solution {
    fn max_rep_opt1(text: String) -> i32 {
        let s: Vec<u8> = text.bytes().collect();
        let n = s.len();
        let mut group: Vec<(u8, usize)> = vec![];
        let mut count: Vec<usize> = vec![0; 26];
        for i in 0..n {
            count[(s[i] - b'a') as usize] += 1;
        }
        let mut prev = 1;
        for i in 1..n {
            if s[i] == s[i - 1] {
                prev += 1;
            } else {
                group.push((s[i - 1], prev));
                prev = 1;
            }
        }
        group.push((s[n - 1], prev));
        let mut res = 0;
        for g in &group {
            res = res.max(count[(g.0 - b'a') as usize].min(g.1 + 1));
        }
        for i in 1..group.len() - 1 {
            if group[i - 1].0 == group[i + 1].0 && group[i].1 == 1 {
                let b = group[i - 1].0;
                res = res.max(count[(b - b'a') as usize].min(group[i - 1].1 + group[i + 1].1 + 1));
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let text = "ababa".to_string();
    let res = 3;
    assert_eq!(Solution::max_rep_opt1(text), res);
    let text = "aaabaaa".to_string();
    let res = 6;
    assert_eq!(Solution::max_rep_opt1(text), res);
    let text = "aaabbaaa".to_string();
    let res = 4;
    assert_eq!(Solution::max_rep_opt1(text), res);
    let text = "aaaaa".to_string();
    let res = 5;
    assert_eq!(Solution::max_rep_opt1(text), res);
    let text = "abcdef".to_string();
    let res = 1;
    assert_eq!(Solution::max_rep_opt1(text), res);
}
