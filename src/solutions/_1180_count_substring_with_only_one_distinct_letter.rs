struct Solution;

impl Solution {
    fn count_letters(s: String) -> i32 {
        let mut prev: Option<char> = None;
        let mut count = 0;
        let mut sum = 0;
        for c in s.chars() {
            if let Some(prev_c) = prev {
                if c == prev_c {
                    count += 1;
                } else {
                    sum += count * (count + 1) / 2;
                    count = 1;
                    prev = Some(c);
                }
            } else {
                count = 1;
                prev = Some(c);
            }
        }
        sum += count * (count + 1) / 2;
        sum as i32
    }
}

#[test]
fn test() {
    let s = "aaaba".to_string();
    assert_eq!(Solution::count_letters(s), 8);
    let s = "aaaaaaaaaa".to_string();
    assert_eq!(Solution::count_letters(s), 55);
}
