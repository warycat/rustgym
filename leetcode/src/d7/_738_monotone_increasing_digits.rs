struct Solution;

impl Solution {
    fn monotone_increasing_digits(n: i32) -> i32 {
        let mut s: Vec<char> = n.to_string().chars().collect();
        let n = s.len();
        let mut i = 1;
        while i < n && s[i] >= s[i - 1] {
            i += 1;
        }
        while i > 0 && i < n && s[i - 1] > s[i] {
            s[i - 1] = (s[i - 1] as u8 - 1) as char;
            i -= 1;
        }
        while i + 1 < n {
            s[i + 1] = '9';
            i += 1;
        }
        s.into_iter().collect::<String>().parse::<i32>().unwrap()
    }
}

#[test]
fn test() {
    let n = 10;
    let res = 9;
    assert_eq!(Solution::monotone_increasing_digits(n), res);
    let n = 1234;
    let res = 1234;
    assert_eq!(Solution::monotone_increasing_digits(n), res);
    let n = 332;
    let res = 299;
    assert_eq!(Solution::monotone_increasing_digits(n), res);
}
