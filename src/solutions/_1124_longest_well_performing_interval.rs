struct Solution;
use std::collections::HashMap;

impl Solution {
    fn longest_wpi(hours: Vec<i32>) -> i32 {
        let n = hours.len();
        let mut hm: HashMap<i32, usize> = HashMap::new();
        let mut score = 0;
        let mut res = 0;
        for i in 0..n {
            score += if hours[i] > 8 { 1 } else { -1 };
            if score > 0 {
                res = i + 1;
            } else {
                hm.entry(score).or_insert(i);
                if let Some(j) = hm.get(&(score - 1)) {
                    res = res.max(i - j);
                }
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let hours = vec![9, 9, 6, 0, 6, 6, 9];
    let res = 3;
    assert_eq!(Solution::longest_wpi(hours), res);
}
