struct Solution;

impl Solution {
    fn minimum_deletions(s: String) -> i32 {
        let n = s.len();
        let mut a = 0;
        let mut b = 0;
        for c in s.chars() {
            if c == 'a' {
                a += 1;
            } else {
                b += 1;
                b = b.max(a + 1);
            }
        }
        (n - a.max(b)) as i32
    }
}

#[test]
fn test() {
    let s = "aababbab".to_string();
    let res = 2;
    assert_eq!(Solution::minimum_deletions(s), res);
    let s = "bbaaaaabb".to_string();
    let res = 2;
    assert_eq!(Solution::minimum_deletions(s), res);
}
