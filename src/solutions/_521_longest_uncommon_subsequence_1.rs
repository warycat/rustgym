struct Solution;

impl Solution {
    fn find_lu_slength(a: String, b: String) -> i32 {
        if a == b {
            -1
        } else {
            a.len().max(b.len()) as i32
        }
    }
}

#[test]
fn test() {
    let a = "aba".to_string();
    let b = "cdc".to_string();
    assert_eq!(Solution::find_lu_slength(a, b), 3);
}
