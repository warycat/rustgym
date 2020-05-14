struct Solution;

use std::cmp::Ordering::*;

impl Solution {
    fn reorder_log_files(mut logs: Vec<String>) -> Vec<String> {
        logs.sort_by(|a, b| {
            let i = a.find(' ').unwrap();
            let j = b.find(' ').unwrap();
            let ar = &a[i + 1..];
            let br = &b[j + 1..];
            let ac = a.chars().last().unwrap();
            let bc = b.chars().last().unwrap();
            match (char::is_digit(ac, 10), char::is_digit(bc, 10)) {
                (true, true) => Equal,
                (true, false) => Greater,
                (false, true) => Less,
                (false, false) => {
                    let ordering = ar.cmp(br);
                    if ordering == Equal {
                        a.cmp(b)
                    } else {
                        ordering
                    }
                }
            }
        });
        logs
    }
}

#[test]
fn test() {
    let input: Vec<String> = vec_string![
        "a1 9 2 3 1",
        "g1 act car",
        "zo4 4 7",
        "ab1 off key dog",
        "a8 act zoo"
    ];
    let output: Vec<String> = vec_string![
        "g1 act car",
        "a8 act zoo",
        "ab1 off key dog",
        "a1 9 2 3 1",
        "zo4 4 7"
    ];
    assert_eq!(Solution::reorder_log_files(input), output);
}
