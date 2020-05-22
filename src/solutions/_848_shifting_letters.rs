struct Solution;

impl Solution {
    fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        let n = s.len();
        let mut s: Vec<char> = s.chars().collect();
        let mut prev: u8 = 0;
        for i in (0..n).rev() {
            prev += (shifts[i] % 26) as u8;
            prev %= 26;
            s[i] = (b'a' + (s[i] as u8 - b'a' + prev) % 26) as char;
        }
        s.into_iter().collect()
    }
}

#[test]
fn test() {
    let s = "abc".to_string();
    let shifts = vec![3, 5, 9];
    let res = "rpl".to_string();
    assert_eq!(Solution::shifting_letters(s, shifts), res);
}
