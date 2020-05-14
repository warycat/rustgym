struct Solution;

impl Solution {
    fn encode(num: i32) -> String {
        format!("{:b}", num + 1)[1..].to_string()
    }
}

#[test]
fn test() {
    let num = 23;
    let res = "1000".to_string();
    assert_eq!(Solution::encode(num), res);
    let num = 107;
    let res = "101100".to_string();
    assert_eq!(Solution::encode(num), res);
}
