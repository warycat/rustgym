struct Solution;

impl Solution {
    fn to_lower_case(s: String) -> String {
        s.chars()
            .map(|c| {
                if c as u8 >= 'A' as u8 && c as u8 <= 'Z' as u8 {
                    (c as u8 + ('a' as u8 - 'A' as u8)) as char
                } else {
                    c
                }
            })
            .collect()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::to_lower_case("Hello".to_string()),
        "hello".to_string()
    );
}
