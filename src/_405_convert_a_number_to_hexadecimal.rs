struct Solution;

#[allow(clippy::wrong_self_convention)]
impl Solution {
    fn to_hex(num: i32) -> String {
        format!("{:x}", num)
    }
}

#[test]
fn test() {
    let num = -1;
    let res = "ffffffff";
    assert_eq!(Solution::to_hex(num), res);
}
