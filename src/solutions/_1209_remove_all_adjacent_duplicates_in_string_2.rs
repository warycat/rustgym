struct Solution;

impl Solution {
    fn remove_duplicates(s: String, k: i32) -> String {
        let mut stack: Vec<(char, usize)> = vec![];
        for c in s.chars() {
            if let Some(top) = stack.pop() {
                if top.0 != c {
                    stack.push(top);
                    stack.push((c, 1));
                } else {
                    if (top.1 + 1) != k as usize {
                        stack.push((c, top.1 + 1));
                    }
                }
            } else {
                stack.push((c, 1));
            }
        }
        let mut res = "".to_string();
        for p in stack {
            for _ in 0..p.1 {
                res.push(p.0);
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "abcd".to_string();
    let k = 2;
    let res = "abcd".to_string();
    assert_eq!(Solution::remove_duplicates(s, k), res);
    let s = "deeedbbcccbdaa".to_string();
    let k = 3;
    let res = "aa".to_string();
    assert_eq!(Solution::remove_duplicates(s, k), res);
    let s = "pbbcggttciiippooaais".to_string();
    let k = 2;
    let res = "ps".to_string();
    assert_eq!(Solution::remove_duplicates(s, k), res);
}
