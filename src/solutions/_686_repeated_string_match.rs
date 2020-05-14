struct Solution;

impl Solution {
    fn repeated_string_match(a: String, b: String) -> i32 {
        let mut s = String::new();
        let n = a.len();
        let m = b.len();
        if n == 0 || m == 0 {
            return -1;
        }
        let mut k = m / n;
        if k * n < m {
            k += 1;
        }
        for _ in 0..k {
            s += &a;
        }
        if s.contains(&b) {
            return k as i32;
        }
        s += &a;
        if s.contains(&b) {
            return (k + 1) as i32;
        }
        -1
    }
}

#[test]
fn test() {
    let a = "abcd".to_string();
    let b = "cdabcdab".to_string();
    assert_eq!(Solution::repeated_string_match(a, b), 3);
}
