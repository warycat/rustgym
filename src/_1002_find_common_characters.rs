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
    let a: Vec<String> = vec_string!["bella", "label", "roller"];
    let b: Vec<String> = vec_string!["e", "l", "l"];
    assert_eq!(Solution::common_chars(a), b);

    let a: Vec<String> = vec_string!["cool", "lock", "cook"];
    let b: Vec<String> = vec_string!["c", "o"];
    assert_eq!(Solution::common_chars(a), b);
}
