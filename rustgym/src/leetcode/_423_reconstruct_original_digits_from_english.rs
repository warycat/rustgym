struct Solution;

use std::collections::HashMap;

impl Solution {
    fn original_digits(s: String) -> String {
        let digits = vec![
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let digits: Vec<HashMap<char, usize>> = digits
            .into_iter()
            .map(|s| {
                let mut hm: HashMap<char, usize> = HashMap::new();
                for c in s.chars() {
                    *hm.entry(c).or_default() += 1;
                }
                hm
            })
            .collect();
        let mut count: HashMap<char, usize> = HashMap::new();
        for c in s.chars() {
            *count.entry(c).or_default() += 1;
        }
        let mut res: Vec<char> = vec![];
        for i in vec![0, 4, 5, 6, 7, 8, 3, 2, 1, 9].into_iter() {
            let mut min = std::usize::MAX;
            for (&c, &v) in &digits[i] {
                if *count.entry(c).or_default() % v != 0 {
                    continue;
                }
                if *count.entry(c).or_default() / v < min {
                    min = *count.entry(c).or_default() / v;
                }
            }
            if min != std::usize::MAX {
                for (&c, &v) in &digits[i] {
                    *count.entry(c).or_default() -= min * v;
                }
                for _ in 0..min {
                    res.push((b'0' + i as u8) as char);
                }
            }
        }
        res.sort_unstable();
        res.into_iter().collect()
    }
}
#[test]
fn test() {
    let s = "owoztneoer".to_string();
    let res = "012".to_string();
    assert_eq!(Solution::original_digits(s), res);
    let s = "fviefuro".to_string();
    let res = "45".to_string();
    assert_eq!(Solution::original_digits(s), res);
}
