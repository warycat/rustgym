struct Solution;
use std::collections::VecDeque;

impl Solution {
    fn predict_party_victory(senate: String) -> String {
        let mut rqueue = VecDeque::new();
        let mut dqueue = VecDeque::new();
        let n = senate.len();
        for (i, c) in senate.char_indices() {
            if c == 'R' {
                rqueue.push_back(i);
            } else {
                dqueue.push_back(i);
            }
        }
        while !rqueue.is_empty() && !dqueue.is_empty() {
            let r = rqueue.pop_front().unwrap();
            let d = dqueue.pop_front().unwrap();
            if r < d {
                rqueue.push_back(r + n);
            } else {
                dqueue.push_back(d + n);
            }
        }
        if rqueue.is_empty() {
            "Dire".to_string()
        } else {
            "Radiant".to_string()
        }
    }
}

#[test]
fn test() {
    let senate = "RD".to_string();
    let res = "Radiant".to_string();
    assert_eq!(Solution::predict_party_victory(senate), res);
    let senate = "RDD".to_string();
    let res = "Dire".to_string();
    assert_eq!(Solution::predict_party_victory(senate), res);
}
