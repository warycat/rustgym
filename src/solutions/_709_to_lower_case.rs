struct Solution;

#[allow(clippy::wrong_self_convention)]
impl Solution {
    fn to_lower_case(s: String) -> String {
        s.chars()
            .map(|c| {
                if c as u8 >= b'A' && c as u8 <= b'Z' {
                    (c as u8 + (b'a' - b'A')) as char
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
