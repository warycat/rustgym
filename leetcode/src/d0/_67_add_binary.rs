struct Solution;

impl Solution {
    fn add_binary(a: String, b: String) -> String {
        let aa = i128::from_str_radix(&a, 2).unwrap_or(0);
        let bb = i128::from_str_radix(&b, 2).unwrap_or(0);
        format!("{:b}", aa + bb)
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::add_binary("1010".to_string(), "1011".to_string()),
        "10101".to_string()
    );
}
