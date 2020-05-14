struct Solution;

impl Solution {
    fn gcd_of_strings(str1: String, str2: String) -> String {
        let n1 = str1.len();
        let n2 = str2.len();
        let mut n = usize::min(n1, n2);
        while n > 0 {
            if n1 % n != 0 || n2 % n != 0 {
                n -= 1;
                continue;
            }
            let s1 = &str1[0..n];
            let s2 = &str2[0..n];
            if s1 != s2 {
                n -= 1;
                continue;
            }
            let k1 = n1 / n;
            let k2 = n2 / n;
            if str1.matches(s1).count() == k1 && str2.matches(s2).count() == k2 {
                return s1.to_string();
            }
            n -= 1;
        }
        "".to_string()
    }
}

#[test]
fn test() {
    let str1 = "ABCABC".to_string();
    let str2 = "ABC".to_string();
    let res = "ABC".to_string();
    assert_eq!(Solution::gcd_of_strings(str1, str2), res);
    let str1 = "ABABAB".to_string();
    let str2 = "ABAB".to_string();
    let res = "AB".to_string();
    assert_eq!(Solution::gcd_of_strings(str1, str2), res);
    let str1 = "LEET".to_string();
    let str2 = "CODE".to_string();
    let res = "".to_string();
    assert_eq!(Solution::gcd_of_strings(str1, str2), res);
}
