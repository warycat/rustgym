struct Solution;
use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    fn reorganize_string(s: String) -> String {
        let n = s.len();
        let mut s: Vec<char> = s.chars().collect();
        let mut hm: HashMap<char, usize> = HashMap::new();
        for &c in s.iter() {
            *hm.entry(c).or_default() += 1;
            if hm[&c] > (n + 1) / 2 {
                return "".to_string();
            }
        }
        s.sort_unstable_by_key(|c| (hm[c], *c));
        s[0..n / 2].reverse();
        let mut queue: VecDeque<char> = VecDeque::new();
        for c in s {
            queue.push_back(c);
        }
        let mut i = 0;
        let mut res = "".to_string();
        while !queue.is_empty() {
            if i % 2 == 0 {
                res.push(queue.pop_back().unwrap());
            } else {
                res.push(queue.pop_front().unwrap());
            }
            i += 1;
        }
        res
    }
}

#[test]
fn test() {
    let s = "aab".to_string();
    let res = "aba".to_string();
    assert_eq!(Solution::reorganize_string(s), res);
    let s = "aaab".to_string();
    let res = "".to_string();
    assert_eq!(Solution::reorganize_string(s), res);
}
