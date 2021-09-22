struct Solution;

impl Solution {
    fn largest_odd_number(num: String) -> String {
        let n = num.len();
        let s: Vec<char> = num.chars().collect();
        for i in (0..n).rev() {
            if (s[i] as u8 - b'0') % 2 == 1 {
                return s[0..=i].iter().collect();
            }
        }
        "".to_string()
    }
}

#[test]
fn test() {
    let num = "52".to_string();
    let res = "5".to_string();
    assert_eq!(Solution::largest_odd_number(num), res);
    let num = "4206".to_string();
    let res = "".to_string();
    assert_eq!(Solution::largest_odd_number(num), res);
    let num = "35427".to_string();
    let res = "35427".to_string();
    assert_eq!(Solution::largest_odd_number(num), res);
}
