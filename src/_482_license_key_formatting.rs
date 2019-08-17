struct Solution;

impl Solution {
    fn license_key_formatting(s: String, k: i32) -> String {
        let mut res: Vec<char> = vec![];
        let mut i = 0;
        for c in s.chars().rev() {
            if c != '-' {
                res.push(c);
                i += 1;
                if i == k {
                    i = 0;
                    res.push('-');
                }
            }
        }
        if let Some(&'-') = res.last() {
            res.pop();
        }
        res.iter().rev().collect::<String>().to_ascii_uppercase()
    }
}

#[test]
fn test() {
    let s = "5F3Z-2e-9-w".to_string();
    let k = 4;
    let o = "5F3Z-2E9W".to_string();
    assert_eq!(Solution::license_key_formatting(s, k), o);

    let s = "2-5g-3-J".to_string();
    let k = 2;
    let o = "2-5G-3J".to_string();
    assert_eq!(Solution::license_key_formatting(s, k), o);
}
