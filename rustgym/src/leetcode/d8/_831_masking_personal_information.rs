struct Solution;

impl Solution {
    fn mask_pii(s: String) -> String {
        if let Some(i) = s.find('@') {
            let s = s.to_lowercase();
            format!("{}*****{}", &s[0..1], &s[i - 1..])
        } else {
            let digits: String = s.chars().filter(|&c| ('0'..='9').contains(&c)).collect();
            let n = digits.len();
            match digits.len() {
                13 => format!("+***-***-***-{}", &digits[n - 4..]),
                12 => format!("+**-***-***-{}", &digits[n - 4..]),
                11 => format!("+*-***-***-{}", &digits[n - 4..]),
                _ => format!("***-***-{}", &digits[n - 4..]),
            }
        }
    }
}

#[test]
fn test() {
    let s = "LeetCode@LeetCode.com".to_string();
    let res = "l*****e@leetcode.com".to_string();
    assert_eq!(Solution::mask_pii(s), res);
    let s = "AB@qq.com".to_string();
    let res = "a*****b@qq.com".to_string();
    assert_eq!(Solution::mask_pii(s), res);
    let s = "1(234)567-890".to_string();
    let res = "***-***-7890".to_string();
    assert_eq!(Solution::mask_pii(s), res);
    let s = "86-(10)12345678".to_string();
    let res = "+**-***-***-5678".to_string();
    assert_eq!(Solution::mask_pii(s), res);
}
