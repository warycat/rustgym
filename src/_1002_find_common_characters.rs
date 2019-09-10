struct Solution;

use std::usize;

impl Solution {
    fn common_chars(a: Vec<String>) -> Vec<String> {
        let n = a.len();
        let mut counts: Vec<Vec<usize>> = vec![vec![0; 256]; n];
        for i in 0..n {
            let w = &a[i];
            for c in w.chars() {
                counts[i][c as usize] += 1;
            }
        }
        let mut res: Vec<String> = vec![];
        for i in 0..26 {
            let c: u8 = b'a' + i;
            let mut min = usize::MAX;
            for j in 0..n {
                let count = counts[j][c as usize];
                min = usize::min(count, min);
            }
            for _ in 0..min {
                res.push(format!("{}", c as char))
            }
        }
        res
    }
}

#[test]
fn test() {
    let a: Vec<String> = vec!["bella", "label", "roller"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let b: Vec<String> = vec!["e", "l", "l"].iter().map(|s| s.to_string()).collect();
    assert_eq!(Solution::common_chars(a), b);

    let a: Vec<String> = vec!["cool", "lock", "cook"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let b: Vec<String> = vec!["c", "o"].iter().map(|s| s.to_string()).collect();
    assert_eq!(Solution::common_chars(a), b);
}
