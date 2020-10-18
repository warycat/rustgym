struct Solution;

use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let n = s.len();
        let a = a as u8;
        let b = b as usize;
        let s: Vec<u8> = s.bytes().map(|b| b - b'0').collect();
        let mut visited: HashSet<Vec<u8>> = HashSet::new();
        let mut queue: VecDeque<Vec<u8>> = VecDeque::new();
        visited.insert(s.clone());
        queue.push_back(s.clone());
        let mut min = s;
        while let Some(first) = queue.pop_front() {
            if first < min {
                min = first.clone();
            }
            let mut rotated = first.clone();
            let plus = Self::add(first, a, n);
            rotated.rotate_right(b);
            if visited.insert(plus.clone()) {
                queue.push_back(plus);
            }
            if visited.insert(rotated.clone()) {
                queue.push_back(rotated);
            }
        }
        let mut res = "".to_string();
        for b in min {
            res.push((b + b'0') as char);
        }
        res
    }

    fn add(mut s: Vec<u8>, a: u8, n: usize) -> Vec<u8> {
        for i in 0..n {
            if i % 2 == 1 {
                s[i] += a;
                s[i] %= 10;
            }
        }
        s
    }
}

#[test]
fn test() {
    let s = "5525".to_string();
    let a = 9;
    let b = 2;
    let res = "2050".to_string();
    assert_eq!(Solution::find_lex_smallest_string(s, a, b), res);
    let s = "74".to_string();
    let a = 5;
    let b = 1;
    let res = "24".to_string();
    assert_eq!(Solution::find_lex_smallest_string(s, a, b), res);
    let s = "0011".to_string();
    let a = 4;
    let b = 2;
    let res = "0011".to_string();
    assert_eq!(Solution::find_lex_smallest_string(s, a, b), res);
    let s = "43987654".to_string();
    let a = 7;
    let b = 3;
    let res = "00553311".to_string();
    assert_eq!(Solution::find_lex_smallest_string(s, a, b), res);
}
