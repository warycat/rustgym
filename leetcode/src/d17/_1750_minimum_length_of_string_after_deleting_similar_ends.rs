struct Solution;

use std::collections::VecDeque;

impl Solution {
    fn minimum_length(s: String) -> i32 {
        let mut queue: VecDeque<char> = s.chars().collect();
        while !queue.is_empty() {
            let size = queue.len();
            if size == 1 {
                break;
            }
            let c = *queue.front().unwrap();
            if queue.front() == queue.back() {
                while let Some(&x) = queue.front() {
                    if x == c {
                        queue.pop_front();
                    } else {
                        break;
                    }
                }
                while let Some(&x) = queue.back() {
                    if x == c {
                        queue.pop_back();
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        queue.len() as i32
    }
}

#[test]
fn test() {
    let s = "ca".to_string();
    let res = 2;
    assert_eq!(Solution::minimum_length(s), res);
    let s = "cabaabac".to_string();
    let res = 0;
    assert_eq!(Solution::minimum_length(s), res);
    let s = "aabccabba".to_string();
    let res = 3;
    assert_eq!(Solution::minimum_length(s), res);
}
