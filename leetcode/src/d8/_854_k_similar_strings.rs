struct Solution;

use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    fn k_similarity(a: String, b: String) -> i32 {
        let n = a.len();
        let a: Vec<char> = a.chars().collect();
        let b: Vec<char> = b.chars().collect();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        visited.insert(a.clone());
        queue.push_back(a);
        let mut res = 0;
        while !queue.is_empty() {
            'outer: for _ in 0..queue.len() {
                let mut front = queue.pop_front().unwrap();
                let mut i = 0;
                while i < n && front[i] == b[i] {
                    i += 1;
                }
                if i == n {
                    return res;
                } else {
                    for j in i + 1..n {
                        if front[j] == b[i] && front[i] == b[j] {
                            front.swap(i, j);
                            if visited.insert(front.clone()) {
                                queue.push_back(front.clone());
                            }
                            front.swap(i, j);
                            continue 'outer;
                        }
                    }
                    for j in i + 1..n {
                        if front[j] == b[i] {
                            front.swap(i, j);
                            if visited.insert(front.clone()) {
                                queue.push_back(front.clone());
                            }
                            front.swap(i, j);
                        }
                    }
                }
            }
            res += 1;
        }
        0
    }
}

#[test]
fn test() {
    let a = "ab".to_string();
    let b = "ba".to_string();
    let res = 1;
    assert_eq!(Solution::k_similarity(a, b), res);
    let a = "abc".to_string();
    let b = "bca".to_string();
    let res = 2;
    assert_eq!(Solution::k_similarity(a, b), res);
    let a = "abac".to_string();
    let b = "baca".to_string();
    let res = 2;
    assert_eq!(Solution::k_similarity(a, b), res);
    let a = "aabc".to_string();
    let b = "abca".to_string();
    let res = 2;
    assert_eq!(Solution::k_similarity(a, b), res);
}
