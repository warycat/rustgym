struct Solution;

impl Solution {
    fn title_to_number(s: String) -> i32 {
        s.bytes()
            .fold(0, |sum, c| sum * 26 + i32::from(c) - 'A' as i32 + 1)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::title_to_number("A".to_string()), 1);
    assert_eq!(Solution::title_to_number("AB".to_string()), 28);
    assert_eq!(Solution::title_to_number("ZY".to_string()), 701);
}
