pub struct Solution;

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        let mut s: String = std::iter::repeat('a').take((n - 1) as usize).collect();
        s.push(if n % 2 == 0 { 'b' } else { 'a' });
        s
    }
}

#[test]
fn test() {
    let n = 4;
    let res = "aaab".to_string();
    assert_eq!(Solution::generate_the_string(n), res);
    let n = 7;
    let res = "aaaaaaa".to_string();
    assert_eq!(Solution::generate_the_string(n), res);
}
