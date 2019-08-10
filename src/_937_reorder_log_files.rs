struct Solution;

use std::cmp::Ordering::*;

impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut digits: Vec<String> = vec![];
        let mut letters: Vec<String> = vec![];
        for log in logs {
            let c: char = log.chars().last().unwrap();
            if char::is_digit(c, 10) {
                digits.push(log);
            } else {
                letters.push(log);
            }
        }

        letters.sort_by(|a, b| {
            let i = a.find(' ').unwrap();
            let j = b.find(' ').unwrap();
            let ar = &a[i + 1..];
            let br = &b[j + 1..];
            let ordering = ar.cmp(br);
            if ordering == Equal {
                a.cmp(b)
            } else {
                ordering
            }
        });
        let mut res: Vec<String> = vec![];
        println!("{:?} {:?}", letters, digits);
        res.append(&mut letters);
        res.append(&mut digits);
        res
    }
}

#[test]
fn test() {
    let input: Vec<String> = [
        "a1 9 2 3 1",
        "g1 act car",
        "zo4 4 7",
        "ab1 off key dog",
        "a8 act zoo",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let output: Vec<String> = [
        "g1 act car",
        "a8 act zoo",
        "ab1 off key dog",
        "a1 9 2 3 1",
        "zo4 4 7",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    assert_eq!(Solution::reorder_log_files(input), output);
}
