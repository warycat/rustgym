struct Solution;

impl Solution {
    fn strong_password_checker(s: String) -> i32 {
        let mut missing = 3;
        let s: Vec<char> = s.chars().collect();
        if s.iter().any(|c| c.is_lowercase()) {
            missing -= 1;
        }
        if s.iter().any(|c| c.is_uppercase()) {
            missing -= 1;
        }
        if s.iter().any(|c| c.is_digit(10)) {
            missing -= 1;
        }
        let mut replace = 0;
        let mut one = 0;
        let mut two = 0;
        let mut three = 0;
        let mut i = 2;
        while i < s.len() {
            if s[i] == s[i - 1] && s[i] == s[i - 2] {
                let mut length = 2;
                while i < s.len() && s[i] == s[i - 1] {
                    length += 1;
                    i += 1;
                }
                replace += length / 3;
                if length % 3 == 0 {
                    one += 1;
                } else if length % 3 == 1 {
                    two += 1;
                } else {
                    three += 1;
                }
            } else {
                i += 1;
            }
        }
        if s.len() < 6 {
            return missing.max(6 - s.len()) as i32;
        }
        if s.len() <= 20 {
            return missing.max(replace) as i32;
        }

        let delete = s.len() - 20;
        let delete_one = one.min(delete);
        let delete_one_left = delete - delete_one;
        let delete_two = (two * 2).min(delete_one_left);
        let delete_two_left = delete_one_left - delete_two;
        let delete_three = ((one + two + three) * 3).min(delete_two_left);
        replace -= delete_one + delete_two / 2 + delete_three / 3;
        (delete + missing.max(replace)) as i32
    }
}

#[test]
fn test() {
    let s = "aaa111".to_string();
    let res = 2;
    assert_eq!(Solution::strong_password_checker(s), res);
    let s = "ABABABABABABABABABAB1".to_string();
    let res = 2;
    assert_eq!(Solution::strong_password_checker(s), res);
    let s = "1Abababcaaaabababababa".to_string();
    let res = 2;
    assert_eq!(Solution::strong_password_checker(s), res);
}
