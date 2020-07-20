struct Solution;

impl Solution {
    fn query_string(s: String, n: i32) -> bool {
        for i in (1..=n).rev() {
            let t = format!("{:b}", i);
            if !s.contains(&t) {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    let s = "0110".to_string();
    let n = 3;
    let res = true;
    assert_eq!(Solution::query_string(s, n), res);
    let s = "0110".to_string();
    let n = 4;
    let res = false;
    assert_eq!(Solution::query_string(s, n), res);
}
