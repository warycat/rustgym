struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

impl Solution {
    fn rearrange_string(s: String, k: i32) -> String {
        let mut count = vec![0; 26];
        let k = k as usize;
        let mut queue: BinaryHeap<(usize, Reverse<char>)> = BinaryHeap::new();
        let mut waiting: VecDeque<(usize, char)> = VecDeque::new();
        let mut index = vec![0; 26];
        for b in s.bytes() {
            count[(b - b'a') as usize] += 1;
        }

        for i in 0..26 {
            if count[i] > 0 {
                queue.push((count[i], Reverse((b'a' + i as u8) as char)));
            }
        }

        let mut res = "".to_string();
        while !queue.is_empty() {
            let mut found = false;
            while let Some((count, Reverse(c))) = queue.pop() {
                if index[(c as u8 - b'a') as usize] > res.len() {
                    waiting.push_back((count, c));
                } else {
                    found = true;
                    index[(c as u8 - b'a') as usize] = res.len() + k;
                    res.push((c) as char);
                    if count - 1 > 0 {
                        waiting.push_back((count - 1, c));
                    }
                    break;
                }
            }
            if !found {
                return "".to_string();
            }
            while let Some((count, c)) = waiting.pop_front() {
                queue.push((count, Reverse(c)));
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "aabbcc".to_string();
    let k = 3;
    let res = "abcabc".to_string();
    assert_eq!(Solution::rearrange_string(s, k), res);
    let s = "aaabc".to_string();
    let k = 3;
    let res = "".to_string();
    assert_eq!(Solution::rearrange_string(s, k), res);
    let s = "aaadbbcc".to_string();
    let k = 2;
    let res = "abacabcd".to_string();
    assert_eq!(Solution::rearrange_string(s, k), res);
}
