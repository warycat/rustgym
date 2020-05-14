struct Solution;

impl Solution {
    fn is_long_pressed_name(name: String, typed: String) -> bool {
        let n = name.len();
        let m = typed.len();
        let mut i = 0;
        let mut j = 0;
        let s: Vec<char> = name.chars().collect();
        let t: Vec<char> = typed.chars().collect();
        while i < n && j < m {
            if s[i] == t[j] {
                let mut a = 0;
                while i + 1 < n && s[i + 1] == s[i] {
                    i += 1;
                    a += 1;
                }
                i += 1;
                let mut b = 0;
                while j + 1 < m && t[j + 1] == t[j] {
                    j += 1;
                    b += 1;
                }
                j += 1;
                if a > b {
                    return false;
                }
            } else {
                return false;
            }
        }
        i == n && j == m
    }
}

#[test]
fn test() {
    let name = "alex".to_string();
    let typed = "aaleex".to_string();
    assert_eq!(Solution::is_long_pressed_name(name, typed), true);
    let name = "saeed".to_string();
    let typed = "ssaaedd".to_string();
    assert_eq!(Solution::is_long_pressed_name(name, typed), false);
    let name = "leelee".to_string();
    let typed = "lleeelee".to_string();
    assert_eq!(Solution::is_long_pressed_name(name, typed), true);
    let name = "laiden".to_string();
    let typed = "laiden".to_string();
    assert_eq!(Solution::is_long_pressed_name(name, typed), true);
    let name = "vtkgn".to_string();
    let typed = "vttkgnn".to_string();
    assert_eq!(Solution::is_long_pressed_name(name, typed), true);
}
