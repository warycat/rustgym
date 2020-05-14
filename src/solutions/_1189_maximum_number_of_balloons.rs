struct Solution;

use std::collections::HashMap;

impl Solution {
    fn max_number_of_balloons(text: String) -> i32 {
        let mut text_count: HashMap<char, usize> = HashMap::new();
        let mut ballon_count: HashMap<char, usize> = HashMap::new();
        for c in "balloon".chars() {
            *ballon_count.entry(c).or_default() += 1;
        }
        for c in text.chars() {
            *text_count.entry(c).or_default() += 1;
        }

        let mut min = text.len();
        for (c, v) in ballon_count {
            min = usize::min(text_count.get(&c).unwrap_or(&0) / v, min);
        }
        min as i32
    }
}

#[test]
fn test() {
    let text = "nlaebolko".to_string();
    assert_eq!(Solution::max_number_of_balloons(text), 1);
    let text = "loonbalxballpoon".to_string();
    assert_eq!(Solution::max_number_of_balloons(text), 2);
    let text = "leetcode".to_string();
    assert_eq!(Solution::max_number_of_balloons(text), 0);
}
